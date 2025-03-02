use yew::prelude::*;
use std::collections::VecDeque;
use crate::models::FileData;
use crate::handlers::{handle_cut_length_input, handle_input_mode_change, handle_file_select
, handle_thickness_input, handle_thickness_select, handle_cutting_type_change, handle_material_change,
handle_clear_thickness};




#[function_component(App)]
pub fn app() -> Html {
    let selected_cutting = use_state(|| String::new());
    let selected_material = use_state(|| String::new());
    let history = use_state(|| VecDeque::<String>::with_capacity(30));
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

  

    // Обработчик для выбора файла
    let handle_file_select = handle_file_select(
    //file_path.clone(),
        history.clone(),
        file_data.clone()       
    );

    // Обработчик ввода длины реза
    let handle_cut_length_input = handle_cut_length_input(
        cut_length.clone(),
        history.clone()
    );

    //Обработчик для переключения режима ввода
    let handle_input_mode_change = handle_input_mode_change(
        is_file_selected.clone(),
        is_manual_input.clone(),
        manual_path.clone(),
        file_path.clone()
    );
    
    // Обработчик изменения ввода толщины
    let handle_thickness_input = handle_thickness_input(
        thickness_input.clone(),
        filtered_thicknesses.clone(),
        history.clone(),
        all_thicknesses.clone()
    );

    // В обработчике выбора толщины
    let handle_thickness_select = handle_thickness_select(
        history.clone(),
        thickness_input.clone(),
        filtered_thicknesses.clone()
        
    );

    // Обработчик для очистки толщины
    let handle_clear = handle_clear_thickness(
        thickness_input.clone(),
        filtered_thicknesses.clone(),
        history.clone()
    );


    // Обработчик для выбора типа резки
    let handle_radio_change = handle_cutting_type_change(
        selected_cutting.clone(),
        history.clone()
    );

    // Обработчик для изменения материала
    let handle_material_change = handle_material_change(
        selected_material.clone(),
        history.clone()
    );

    html! {
        <main class="container">
            <div class="main-content">
                <h1>{"Расчет стоимости резки"}</h1>

                <div class="cutting-types">
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

                <div class="material-types">
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
            </div>

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

