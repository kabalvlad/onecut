
use yew::prelude::*;
use crate::models::AppState;
use crate::components::MainContent;
use crate::handlers::{handle_thickness_select, handle_material_change, handle_cutting_type_change,
handle_thickness_input, handle_clear_thickness, handle_input_mode_change, 
handle_cut_length_input, handle_file_select, handle_bending_points_radio_change,
handle_bending_points_input_change, handle_threads_inserts_mats_radio_change,
handle_threads_inserts_mats_input_change, handle_parts_count_change,
handle_material_price_change, handle_margin_change, handle_clear_options,
handle_get_price_one_part, handle_get_price_all_parts};

#[function_component(App)]
pub fn app() -> Html {
    
    // Используем редуктор вместо множества use_state
    let state = use_reducer(|| AppState::default());
    let state_for_effect = state.clone();



    // Определяем все доступные толщины материала
    let all_thicknesses: Vec<f32> = vec![
        0.5, 0.7, 0.8, 1.0, 1.2, 1.5, 2.0, 2.5, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0, 10.0, 12.0, 14.0, 15.0, 16.0, 
        20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0
    ];
    
    // Клонируем вектор для использования в JSX
    let all_thicknesses_for_jsx = all_thicknesses.clone();

    let handle_cutting_type_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_cutting_type_change(state.clone()).emit(e);
        })
    };
    
    let handle_material_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_material_change(state.clone()).emit(e);
        })
    };

    let handle_thickness_select = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_thickness_select(state.clone()).emit(e);
        })
    };
    
    let handle_thickness_input = {
        let state = state.clone();
        let thicknesses = all_thicknesses.clone(); // Клонируем для использования в замыкании
        Callback::from(move |e: Event| {
            handle_thickness_input(state.clone(), thicknesses.clone()).emit(e);
        })
    };

    let handle_clear_thickness = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            handle_clear_thickness(state.clone()).emit(e);
        })
    };
    
    
    let handle_input_mode_change = {
        let state = state.clone();
        Callback::from(move |mode: String| {  
            handle_input_mode_change(state.clone()).emit(mode);
        })
    };  
    
    let handle_cut_length_input = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_cut_length_input(state.clone()).emit(e);
        })
    };

    let handle_file_select = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_file_select(state.clone()).emit(e);
        })
    }; 

    
    let handle_bending_points_radio_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_bending_points_radio_change(state.clone()).emit(e);
        })
    };
    
    let handle_bending_points_input_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_bending_points_input_change(state.clone()).emit(e);
        })
    };
    
    let handle_threads_inserts_mats_radio_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_threads_inserts_mats_radio_change(state.clone()).emit(e);
        })
    };
    
    let handle_threads_inserts_mats_input_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_threads_inserts_mats_input_change(state.clone()).emit(e);
        })
    };
    
    let handle_parts_count_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_parts_count_change(state.clone()).emit(e);
        })
    };
    
    let handle_material_price_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_material_price_change(state.clone()).emit(e);
        })
    };
    
    let handle_margin_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            handle_margin_change(state.clone()).emit(e);
        })
    };    
    
    let handle_clear_options = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            handle_clear_options(state.clone()).emit(e);
        })
    };

    
    // Эффект для расчета цен при изменении параметров
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
            if !cutting_type.is_empty() && !material.is_empty() && !thickness.is_empty() && 
                *cut_length > 0.0 && *parts_count > 0 && *margin >= 0 && *material_price > 0.0 {
                
                // Вызываем обработчики получения цен
                handle_get_price_one_part(state_for_effect.clone()).emit(());
                handle_get_price_all_parts(state_for_effect.clone()).emit(());
            }
                     
            // Возвращаем функцию очистки (в данном случае пустую)
            || ()
        }
    );

    html! {
        <MainContent
            state={state}
            all_thicknesses={all_thicknesses_for_jsx}
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
            handle_input_mode_change={handle_input_mode_change}
            handle_file_select={handle_file_select}
            
        />
    }  
}
