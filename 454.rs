use yew::prelude::*;
use std::rc::Rc;
use crate::models::AppState;
use std::collections::VecDeque;
use crate::models::FileData;
use crate::handlers::{handle_cut_length_input, handle_input_mode_change, handle_file_select
, handle_thickness_input, handle_thickness_select, handle_cutting_type_change, handle_material_change,
handle_clear_thickness, handle_bending_points_radio_change, handle_bending_points_input_change,
handle_threads_inserts_mats_radio_change, handle_threads_inserts_mats_input_change,
handle_clear_options, handle_margin_change, handle_material_price_change,
 handle_parts_count_change, handle_get_price_one_part, handle_get_price_all_parts};

#[function_component(App)]
pub fn app() -> Html {
    let selected_cutting = use_state(|| String::new());
    let selected_material = use_state(|| String::new());
    let thickness_input = use_state(|| String::new());
    let filtered_thicknesses = use_state(|| Vec::new());
    let all_thicknesses: Vec<f32> = vec![
        0.5, 0.7, 0.8, 1.0, 1.2, 1.5, 2.0, 2.5, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0, 10.0, 12.0, 14.0, 15.0, 16.0, 
        20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0
    ];
    let file_path = use_state(|| String::new());
    let manual_path = use_state(|| String::new());
    let is_file_selected = use_state(|| false);
    let is_manual_input = use_state(|| false);
    let cut_length = use_state(|| 0.0f32); 
    let file_data = use_state(|| None::<FileData>);
    let bending_points_radio_state = use_state(|| String::from("no"));
    let threads_inserts_mats_radio_state = use_state(|| String::from("no"));
    let parts_count = use_state(|| 1);
    let material_price = use_state(|| 0.0);
    let margin = use_state(|| 28);
    let price_per_part = use_state(|| 0.0);
    let price_total = use_state(|| 0.0);
    let history = use_state(|| VecDeque::<String>::with_capacity(30));
    let threads_inserts_mats_input = use_state(|| String::new());
    let bending_points_input = use_state(|| String::new());

    

    let app_state = Rc::new(AppState {
        selected_cutting: selected_cutting.clone(),
        selected_material: selected_material.clone(),
        thickness_input: thickness_input.clone(),
        filtered_thicknesses: filtered_thicknesses.clone(),
        file_path: file_path.clone(),
        manual_path: manual_path.clone(),
        is_file_selected: is_file_selected.clone(),
        is_manual_input: is_manual_input.clone(),
        cut_length: cut_length.clone(),
        file_data: file_data.clone(),
        bending_points_radio_state: bending_points_radio_state.clone(),
        threads_inserts_mats_radio_state: threads_inserts_mats_radio_state.clone(),
        bending_points_input: bending_points_input.clone(),
        threads_inserts_mats_input: threads_inserts_mats_input.clone(),
        parts_count: parts_count.clone(),
        material_price: material_price.clone(),
        margin: margin.clone(),
        history: history.clone(),
        price_per_part: price_per_part.clone(),
        price_total: price_total.clone(),         
    });    


    // Обработчик для выбора файла
    let handle_file_select = handle_file_select(
        Rc::clone(&app_state)
    );
    

    // Обработчик ввода длины реза
    let handle_cut_length_input = handle_cut_length_input(
        Rc::clone(&app_state)
    );

    //Обработчик для переключения режима ввода
    let handle_input_mode_change = handle_input_mode_change(
        Rc::clone(&app_state)
    );
    
    // Обработчик изменения ввода толщины
    let handle_thickness_input = handle_thickness_input(
        Rc::clone(&app_state),
        all_thicknesses.clone()
    );

    // В обработчике выбора толщины
    let handle_thickness_select = handle_thickness_select(
        Rc::clone(&app_state),
        filtered_thicknesses.clone()
            
    );

    // Обработчик для очистки толщины
    let handle_clear = handle_clear_thickness(
        Rc::clone(&app_state),
        filtered_thicknesses.clone()
    );

    // Обработчик для выбора типа резки
    let handle_radio_change = handle_cutting_type_change(
        Rc::clone(&app_state)
    );

    // Обработчик для изменения материала
    let handle_material_change = handle_material_change(
        Rc::clone(&app_state)
    );


    // Обработчик для количества точек сгиба    
    
    let handle_bending_points_input_change = handle_bending_points_input_change(
        Rc::clone(&app_state)
    );

    let handle_bending_points_radio_change = handle_bending_points_radio_change(
        Rc::clone(&app_state)
    );
    // Обработчик для количества резьбы вставко и цековок 

    
    let handle_threads_inserts_mats_input_change = handle_threads_inserts_mats_input_change(
        Rc::clone(&app_state)
    );

    let handle_threads_inserts_mats_radio_change = handle_threads_inserts_mats_radio_change(
        Rc::clone(&app_state)
    );   
 
    // Обработчик изменения количества деталей
    let handle_parts_count_change = handle_parts_count_change (
        Rc::clone(&app_state)
    );

   
    // Обработчик изменения цены материала
    let handle_material_price_change = handle_material_price_change(
        Rc::clone(&app_state)
    );


    // Обработчик изменения маржи
    let handle_margin_change = handle_margin_change(
        Rc::clone(&app_state)
    );

    // Обработчик очистки полей
    let handle_clear_options = handle_clear_options(
        Rc::clone(&app_state)
    );

    let _handle_get_price_one_part = handle_get_price_one_part(
        price_per_part.clone()
    );

    let _handle_get_price_all_parts = handle_get_price_all_parts(
        price_total.clone()
    );
    
    html! {
        <main class="container">
            <div class="main-content">
                <h1>{"Расчет стоимости резки"}</h1>
                    <div class="container-row">
                    <div class="cutting-types column">
                        <h2>{"Выберите тип резки"}</h2>
                        <div class="cutting-options">
                            <div class="cutting-type">
                                <label>
                                    <input
                                        type="radio"
                                        name="cutting"
                                        value="laser"
                                        onchange={handle_radio_change.clone()}
                                    />
                                    {"Лазерная резка"}
                                </label>
                            </div>
                            <div class="cutting-type">
                                <label>
                                    <input
                                        type="radio"
                                        name="cutting"
                                        value="plasma"
                                        onchange={handle_radio_change.clone()}
                                    />
                                    {"Плазменная резка"}
                                </label>
                            </div>
                            <div class="cutting-type">
                                <label>
                                    <input
                                        type="radio"
                                        name="cutting"
                                        value="hydro"
                                        onchange={handle_radio_change.clone()}
                                    />
                                    {"Гидроабразивная резка"}
                                </label>
                            </div>
                        </div>
                    </div>
                
                    <div class="material-types column">
                        <h2>{"Выберите материал"}</h2>
                        <div class="material-options">
                            <div class="material-type">
                                <label>
                                    <input
                                        type="radio"
                                        name="material"
                                        value="aluminum"
                                        onchange={handle_material_change.clone()}
                                    />
                                    {"Алюминий"}
                                </label>
                            </div>
                            <div class="material-type">
                                <label>
                                    <input
                                        type="radio"
                                        name="material"
                                        value="steel"
                                        onchange={handle_material_change.clone()}
                                    />
                                    {"Сталь"}
                                </label>
                            </div>
                            <div class="material-type">
                                <label>
                                    <input
                                        type="radio"
                                        name="material"
                                        value="stainless"
                                        onchange={handle_material_change.clone()}
                                    />
                                    {"Нержавеющая сталь"}
                                </label>
                            </div>
                            <div class="material-type">
                                <label>
                                    <input
                                        type="radio"
                                        name="material"
                                        value="copper"
                                        onchange={handle_material_change.clone()}
                                    />
                                    {"Латунь/Бронза/Медь"}
                                </label>
                            </div>
                            <div class="material-type">
                                <label>
                                    <input
                                        type="radio"
                                        name="material"
                                        value="plastic"
                                        onchange={handle_material_change.clone()}
                                    />
                                    {"Пластик"}
                                </label>
                            </div>
                        </div>
                    </div>
                </div>
            
                

                <div class="thickness-selector">
                    <h2>{"Выберите толщину материала"}</h2>
                    <div class="thickness-input-container">
                        <input
                            type="text"
                            placeholder="Введите толщину в мм"
                            value={(*thickness_input).clone()}
                            onchange={handle_thickness_input.clone()}
                        />
                        <select onchange={handle_thickness_select}>
                            <option value="">{"Выберите толщину"}</option>
                            {
                                if filtered_thicknesses.is_empty() {
                                    all_thicknesses.iter().map(|&thickness| {
                                        html! {
                                            <option value={thickness.to_string()}>
                                                {format!("{} мм", thickness)}
                                            </option>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    filtered_thicknesses.iter().map(|&thickness| {
                                        html! {
                                            <option value={thickness.to_string()}>
                                                {format!("{} мм", thickness)}
                                            </option>
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        </select>
                        <button onclick={handle_clear} class="clear-button">
                            {"Очистить"}
                        </button>
                    </div>
                </div>

                <div class="cut-length-section">
                    <h2>{"Длина реза"}</h2>
                    <div class="input-mode-selector">
                        <label>
                            <input 
                                type="radio"
                                name="input-mode"
                                checked={*is_file_selected}
                                onclick={let handle_clone = handle_input_mode_change.clone();
                                    Callback::from(move |_| handle_clone.emit("file"))}
                            />
                            {"Выбрать файл"}
                        </label>
                        <label>
                            <input 
                                type="radio"
                                name="input-mode"
                                checked={*is_manual_input}
                                onclick={let handle_clone = handle_input_mode_change.clone();
                                    Callback::from(move |_| handle_clone.emit("manual"))}
                            />
                            {"Ручной ввод"}
                        </label>
                    </div>
                    <div class="file-input-container">
                        {
                            if *is_file_selected {
                                html! {
                                    <input
                                        type="file"
                                        accept=".dxf"
                                        onchange={handle_file_select}
                                    />
                                }
                            } else {
                                html! {}
                            }
                        }
                        {
                            if *is_manual_input {
                                html! {
                                    <div class="manual-input-container">
                                        <input
                                            type="text"
                                            placeholder="Введите длину реза в мм"
                                            onchange={handle_cut_length_input}
                                        />
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <div class="cut-length-display">
                            <span>{"Длина реза в мм: "}</span>
                            <span>
                                {
                                    if *cut_length > 0.0 {
                                        format!("{:.2} мм", *cut_length)
                                    } else {
                                        "Не рассчитано".to_string()
                                    }
                                }
                            </span>
                        </div>
                    </div>
                </div>
                <div class="bending-points-threads-inserts-mats-section">
                    <div class="section-columns">
                        <div class="section-column">
                            <h2>{"Места гибки"}</h2>
                            <div class="bending-points-options">
                                <div class="radio-group">
                                    <label>
                                        <input 
                                            type="radio"
                                            name="bending-points"
                                            value="no"
                                            checked={*bending_points_radio_state == "no"}
                                            onchange={handle_bending_points_radio_change.clone()}
                                        />
                                        {"Нет"}
                                    </label>
                                    <label>
                                        <input 
                                            type="radio"
                                            name="bending-points"
                                            value="yes"
                                            checked={*bending_points_radio_state == "yes"}
                                            onchange={handle_bending_points_radio_change.clone()}
                                        />
                                        {"Да"}
                                    </label>
                                </div>
                                <div class="number-input-container">
                                    <input 
                                        type="number"
                                        placeholder="Количество"
                                        min="0"
                                        value={(*bending_points_input).clone()}
                                        disabled={*bending_points_radio_state == "no"}
                                        onchange={handle_bending_points_input_change.clone()}
                                    />
                                </div>
                            </div>
                        </div>
                        
                        <div class="section-column">
                            <h2>{r"Резьба \ вставки \ цековки "}</h2>
                            <div class="threads-inserts-mats-options">
                                <div class="radio-group">
                                    <label>
                                        <input 
                                            type="radio"
                                            name="threads-inserts-mats"
                                            value="no"
                                            checked={*threads_inserts_mats_radio_state == "no"}
                                            onchange={handle_threads_inserts_mats_radio_change.clone()}
                                        />
                                        {"Нет"}
                                    </label>
                                    <label>
                                        <input 
                                            type="radio"
                                            name="threads-inserts-mats"
                                            value="yes"
                                            checked={*threads_inserts_mats_radio_state == "yes"}
                                            onchange={handle_threads_inserts_mats_radio_change.clone()}
                                        />
                                        {"Да"}
                                    </label>
                                </div>
                                <div class="number-input-container">
                                    <input 
                                        type="number"
                                        placeholder="Количество"
                                        min="0"
                                        value={(*threads_inserts_mats_input).clone()}
                                        disabled={*threads_inserts_mats_radio_state == "no"}
                                        onchange={handle_threads_inserts_mats_input_change.clone()}
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                //новая секция с другими значениями
                <div class="section-other-options">
                    <div class="options-row">
                        <div class="input-group">
                            <label for="parts-count">{"Количество деталей:"}</label>
                            <input 
                                type="text" 
                                value={(*parts_count).to_string()} 
                                onchange={handle_parts_count_change.clone()} 
                              
                            />
                        </div>
                        <div class="input-group">
                            <label for="material-price">{"Цена материала за деталь (€):"}</label>
                            <input 
                                type="text"
                                placeholder="Введите цену заготовки"  
                                value={(*material_price).to_string()} // Исправлено с parts_count на material_price
                                onchange={handle_material_price_change.clone()} 
                            />
                        </div>
                        
                        <div class="input-group">
                            <label for="margin">{"Маржа заказа (%):"}</label>
                            <input 
                                type="text" 
                                value={(*margin).to_string()}  // Исправлено с parts_count на margin
                                onchange={handle_margin_change.clone()} 
                            />
                        </div>
                    
                                               
                        <button 
                            class="clear-button" 
                            onclick={handle_clear_options}
                        >
                            {"Очистить"}
                        </button>                       
                    </div>
                    <div class="results-row">
                        <div class="result-group">
                            <label>{"Цена за одну деталь (€):"}</label>
                            <div class="result-value" id="price-per-part">
                                {format!("{:.2}", *price_per_part)}
                            </div>
                        </div>                   
                        
                        <div class="result-group">
                            <label>{"Цена за партию деталей (€):"}</label>
                            <div class="result-value" id="price-total">
                                {format!("{:.2}", *price_total)}
                            </div>
                        </div>
                    </div>                    
                </div>
            
            //тут блок заканчиваеться где вводят                              
            </div>
            // тут блок информации начинаеться
            <div class="info-box">
                <h2>{"Информация"}</h2>
                <div class="history-list">
                    {
                        history.iter().map(|message| {
                            html! {
                                <p>{message}</p>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </main>
    }
}



