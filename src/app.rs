// src/app.rs
use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::models::{AppState, AppAction};
use crate::components::MainContent;

#[function_component(App)]
pub fn app() -> Html {
    // Используем редуктор вместо множества use_state
    let state = use_reducer(|| AppState::default());
    
    // Определяем все доступные толщины материала
    let all_thicknesses: Vec<f32> = vec![
        0.5, 0.7, 0.8, 1.0, 1.2, 1.5, 2.0, 2.5, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0, 10.0, 12.0, 14.0, 15.0, 16.0, 
        20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0
    ];

    // Создаем обработчики событий
    let handle_cutting_type_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.dispatch(AppAction::SetCuttingType(input.value()));
        })
    };
    
    let handle_material_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.dispatch(AppAction::SetMaterial(input.value()));
        })
    };
    
    let handle_thickness_input = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.dispatch(AppAction::SetThickness(input.value()));
        })
    };
    
    let handle_thickness_select = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            state.dispatch(AppAction::SetThickness(select.value()));
        })
    };
    
    let handle_clear_thickness = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(AppAction::SetThickness(String::new()));
        })
    };
    
    let handle_input_mode_change = {
        let state = state.clone();
        Callback::from(move |mode: &str| {
            // Здесь нужно добавить действие для изменения режима ввода
            // Пока просто добавим сообщение в историю
            state.dispatch(AppAction::AddHistoryMessage(format!("Режим ввода изменен на: {}", mode)));
        })
    };
    
    let handle_cut_length_input = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(length) = input.value().parse::<f32>() {
                state.dispatch(AppAction::SetCutLength(length));
            }
        })
    };
    
    let handle_bending_points_radio_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let enabled = input.value() == "yes";
            state.dispatch(AppAction::SetBendingPoints { 
                enabled, 
                count: if enabled { state.bending_points_count } else { None } 
            });
        })
    };
    
    let handle_bending_points_input_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(count) = input.value().parse::<i32>() {
                state.dispatch(AppAction::SetBendingPoints { 
                    enabled: state.bending_points_enabled, 
                    count: Some(count) 
                });
            }
        })
    };
    
    let handle_threads_inserts_mats_radio_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let enabled = input.value() == "yes";
            state.dispatch(AppAction::SetThreadsInsertsMats { 
                enabled, 
                count: if enabled { state.threads_inserts_mats_count } else { None } 
            });
        })
    };
    
    let handle_threads_inserts_mats_input_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(count) = input.value().parse::<i32>() {
                state.dispatch(AppAction::SetThreadsInsertsMats { 
                    enabled: state.threads_inserts_mats_enabled, 
                    count: Some(count) 
                });
            }
        })
    };
    
    let handle_parts_count_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(count) = input.value().parse::<i32>() {
                state.dispatch(AppAction::SetPartsCount(count));
            }
        })
    };
    
    let handle_material_price_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().replace(',', ".");
            if let Ok(price) = value.parse::<f32>() {
                state.dispatch(AppAction::SetMaterialPrice(price));
            }
        })
    };
    
    let handle_margin_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(margin) = input.value().parse::<i32>() {
                state.dispatch(AppAction::SetMargin(margin));
            }
        })
    };
    
    let handle_clear_options = {
        let state = state.clone();
        Callback::from(move |_| {
            // Сбрасываем все значения к значениям по умолчанию
            state.dispatch(AppAction::SetPartsCount(1));
            state.dispatch(AppAction::SetMaterialPrice(0.0));
            state.dispatch(AppAction::SetMargin(28));
            state.dispatch(AppAction::UpdatePrices { price_per_part: 0.0, price_total: 0.0 });
            state.dispatch(AppAction::AddHistoryMessage("Опции очищены".to_string()));
        })
    };
    
    // Эффект для расчета цен при изменении параметров
    let state_for_effect = state.clone();
    use_effect_with(
        (
            state.cutting_type.clone(),
            state.material.clone(),
            state.thickness.clone(),
            state.cut_length,
            state.parts_count,
            state.margin,
            state.material_price
        ),
        move |deps| {
            let (cutting_type, material, thickness, cut_length, parts_count, margin, material_price) = deps;
            
            // Проверяем, что все необходимые данные заполнены
            if !cutting_type.is_empty() && !material.is_empty() && !thickness.is_empty() && *cut_length > 0.0 {
                // Здесь должен быть ваш код для расчета цен
                // Для примера просто устанавливаем какие-то значения
                let price_per_part = *cut_length * 0.01 + *material_price;
                let price_total = price_per_part * (*parts_count as f32) * (1.0 + (*margin as f32 / 100.0));
                
                // Диспетчеризуем действие обновления цен
                state_for_effect.dispatch(AppAction::UpdatePrices { 
                    price_per_part, 
                    price_total 
                });
            }
            
            // Возвращаем функцию очистки (в данном случае пустую)
            || ()
        }
    );

    html! {
        <MainContent
            state={state}
            all_thicknesses={all_thicknesses}
            handle_cutting_type_change={handle_cutting_type_change}
            handle_material_change={handle_material_change}
            handle_thickness_input={handle_thickness_input}
            handle_thickness_select={handle_thickness_select}
            handle_clear_thickness={handle_clear_thickness}
            handle_cut_length_input={handle_cut_length_input}
            handle_bending_points_radio_change={handle_bending_points_radio_change}
            handle_bending_points_input_change={handle_bending_points_input_change}
            handle_threads_inserts_mats_radio_change={handle_threads_inserts_mats_radio_change}
            handle_threads_inserts_mats_input_change={handle_threads_inserts_mats_input_change}
            handle_parts_count_change={handle_parts_count_change}
            handle_material_price_change={handle_material_price_change}
            handle_margin_change={handle_margin_change}
            handle_clear_options={handle_clear_options}
        />
    }  
    
    
}
