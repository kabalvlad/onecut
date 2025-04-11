use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use crate::handlers::{
    handle_cutting_type_change, handle_material_change, handle_thickness_select,
    handle_thickness_input, handle_clear_thickness, handle_input_mode_change,
    handle_cut_length_input, handle_file_select, handle_bending_points_radio_change,
    handle_bending_points_input_change, handle_threads_inserts_mats_radio_change,
    handle_threads_inserts_mats_input_change, handle_parts_count_change,
    handle_material_price_change, handle_margin_change, handle_clear_options
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub all_thicknesses: Vec<f32>,
}

#[function_component(MainContent)]
pub fn main_content(props: &Props) -> Html {
    // Состояния для хранения цен
    let price_per_part = use_state(|| 0.0);
    let price_total = use_state(|| 0.0);

    // Добавляем слушатель события price-update
    {
        let price_per_part = price_per_part.clone();
        let price_total = price_total.clone();
        
        use_effect_with(
            (),  // Пустые зависимости, эффект запускается только при монтировании
            move |_| {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                
                // Создаем функцию-обработчик события
                let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    // Преобразуем Event в CustomEvent
                    let custom_event = event.dyn_into::<web_sys::CustomEvent>().unwrap();
                    let detail = custom_event.detail();
                    
                    // Получаем цены из события
                    if let Some(price_per_part_js) = js_sys::Reflect::get(&detail, &"pricePerPart".into()).ok() {
                        if let Some(price) = price_per_part_js.as_f64() {
                            price_per_part.set(price as f32);
                        }
                    }
                    
                    if let Some(price_total_js) = js_sys::Reflect::get(&detail, &"priceTotal".into()).ok() {
                        if let Some(price) = price_total_js.as_f64() {
                            price_total.set(price as f32);
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                
                // Добавляем слушатель события
                document.add_event_listener_with_callback(
                    "price-update",
                    callback.as_ref().unchecked_ref()
                ).unwrap();
                
                // Сохраняем callback, чтобы он не был удален сборщиком мусора
                callback.forget();
                
                // Функция очистки (не используется, так как callback.forget())
                || {}
            }
        );
    }


    html! {
        <main class="container">
            <h1>{"Расчет стоимости резки"}</h1>
            
            // Первый ряд: два блока рядом
            <div class="row-1">
                // Левый блок: типы резки и материалы (в одном ряду)
                <div class="block-left">
                    <div class="block-row">
                        // Типы резки
                        <div class="cutting-types">
                            <h2>{"Выберите тип резки"}</h2>
                            <div class="cutting-options">
                                <div class="cutting-type">
                                    <label>
                                        <input type="radio" name="cutting" value="laser" 
                                            onchange={handle_cutting_type_change()}
                                        />
                                        {"Лазерная резка"}
                                    </label>
                                </div>
                                <div class="cutting-type">
                                    <label>
                                        <input type="radio" name="cutting" value="plasma"
                                            onchange={handle_cutting_type_change()}
                                        />
                                        {"Плазменная резка"}
                                    </label>
                                </div>
                                <div class="cutting-type">
                                    <label>
                                        <input type="radio" name="cutting" value="hydro"
                                            onchange={handle_cutting_type_change()}
                                        />
                                        {"Гидроабразивная резка"}
                                    </label>
                                </div>
                            </div>
                        </div>

                        // Материалы
                        <div class="material-types">
                            <h2>{"Выберите материал"}</h2>
                            <div class="material-options">
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="aluminum"
                                            onchange={handle_material_change()}
                                        />
                                        {"Алюминий"}
                                    </label>
                                </div>
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="steel"
                                            onchange={handle_material_change()}
                                        />
                                        {"Сталь"}
                                    </label>
                                </div>
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="stainless"
                                            onchange={handle_material_change()}
                                        />
                                        {"Нержавеющая сталь"}
                                    </label>
                                </div>
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="copper"
                                            onchange={handle_material_change()}
                                        />
                                        {"Латунь/Бронза/Медь"}
                                    </label>
                                </div>
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="plastic"
                                            onchange={handle_material_change()}
                                        />
                                        {"Пластик"}
                                    </label>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                // Правый блок: толщина и длина реза (в одном ряду)
                <div class="block-right">
                    <div class="block-row">
                        // Толщина
                        <div class="thickness-selector">
                            <h2>{"Выберите толщину материала"}</h2>
                            <div class="thickness-input-container">
                                <input type="text" placeholder="Введите толщину в мм"
                                    onchange={handle_thickness_input(props.all_thicknesses.clone())}
                                />
                                <select onchange={handle_thickness_select()}>
                                    <option value="">{"Выберите толщину"}</option>
                                    {props.all_thicknesses.iter().map(|&thickness| {
                                        html! {
                                            <option value={thickness.to_string()}>
                                                {format!("{} мм", thickness)}
                                            </option>
                                        }
                                    }).collect::<Html>()}
                                </select>
                                <button onclick={handle_clear_thickness()} class="clear-button">
                                    {"Очистить"}
                                </button>
                            </div>
                        </div>

                        // Длина реза
                        <div class="cut-length-section">
                            <h2>{"Длина реза"}</h2>
                            <div class="input-mode-selector">
                                <label>
                                    <input type="radio" name="input-mode" value="file"
                                        onclick={
                                            let handle_mode = handle_input_mode_change();
                                            Callback::from(move |_| handle_mode.emit("file".to_string()))
                                        }
                                    />
                                    {"Выбрать файл"}
                                </label>
                                <label>
                                    <input type="radio" name="input-mode" value="manual"
                                        onclick={
                                            let handle_mode = handle_input_mode_change();
                                            Callback::from(move |_| handle_mode.emit("manual".to_string()))
                                        }
                                    />
                                    {"Ручной ввод"}
                                </label>
                            </div>
                            <div class="file-input-container">
                                <div class="file-input">
                                    <input type="file" id="dxf-file" accept=".dxf"
                                        onchange={handle_file_select()}
                                    />
                                    <label for="dxf-file">{""}</label>
                                </div>
                                <div class="manual-input-container">
                                    <input type="text" placeholder="Введите длину реза в мм"
                                        onchange={handle_cut_length_input()}
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Второй ряд: все четыре секции в одной строке
            <div class="row-2">
                // Секция гибки
                <div class="bending-section">
                    <h2>{"Места гибки"}</h2>
                    <div class="bending-points-options">
                        <div class="radio-group">
                            <label>
                                <input 
                                    type="radio"
                                    name="bending-points"
                                    value="no"
                                    onchange={handle_bending_points_radio_change()}
                                />
                                {"Нет"}
                            </label>
                            <label>
                                <input 
                                    type="radio"
                                    name="bending-points"
                                    value="yes"
                                    onchange={handle_bending_points_radio_change()}
                                />
                                {"Да"}
                            </label>
                        </div>
                        <div class="number-input-container">
                            <input 
                                type="number"
                                placeholder="Количество"
                                min="0"
                                onchange={handle_bending_points_input_change()}
                            />
                        </div>
                    </div>
                </div>

                // Секция резьбы
                <div class="threads-section">
                    <h2>{r"Резьба \ вставки \ цековки"}</h2>
                    <div class="threads-inserts-mats-options">
                        <div class="radio-group">
                            <label>
                                <input 
                                    type="radio"
                                    name="threads-inserts-mats"
                                    value="no"
                                    onchange={handle_threads_inserts_mats_radio_change()}
                                />
                                {"Нет"}
                            </label>
                            <label>
                                <input 
                                    type="radio"
                                    name="threads-inserts-mats"
                                    value="yes"
                                    onchange={handle_threads_inserts_mats_radio_change()}
                                />
                                {"Да"}
                            </label>
                        </div>
                        <div class="number-input-container">
                            <input 
                                type="number"
                                placeholder="Количество"
                                min="0"
                                onchange={handle_threads_inserts_mats_input_change()}
                            />
                        </div>
                    </div>
                </div>

                // Секция других опций
                <div class="other-options">
                    <h2>{"Дополнительные параметры"}</h2>
                    <div class="options-content">
                        // Первая строка: количество и стоимость
                        <div class="options-row">
                            <div class="input-group">
                                <label for="parts-count">{"Количество:"}</label>
                                <input 
                                    type="text" 
                                    onchange={handle_parts_count_change()} 
                                />
                            </div>
                            <div class="input-group">
                                <label for="material-price">{"Стоимость (€):"}</label>
                                <input 
                                    type="text"
                                    placeholder="Цена"  
                                    onchange={handle_material_price_change()} 
                                />
                            </div>
                        </div>
                        // Вторая строка: маржа и кнопка очистить
                        <div class="options-row">
                            <div class="input-group">
                                <label for="margin">{"Маржа (%):"}</label>
                                <input 
                                    type="text" 
                                    onchange={handle_margin_change()} 
                                />
                            </div>
                            <button 
                                class="clear-button" 
                                onclick={handle_clear_options()}
                            >
                                {"Очистить"}
                            </button>
                        </div>
                    </div>
                </div>
                // Секция результатов
                <div class="results-section">
                    <h2>{"Результаты расчета"}</h2>
                    <div class="results-content">
                        <div class="result-group">
                            <label>{"Цена за деталь (€):"}</label>
                            <div class="result-value" id="price-per-part">
                                {format!("{:.2}", *price_per_part)}
                            </div>
                        </div>
                        <div class="result-group">
                            <label>{"Цена за партию (€):"}</label>
                            <div class="result-value" id="price-total">
                                {format!("{:.2}", *price_total)}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Информационный блок (можно заменить на консоль или убрать)
            <div class="info-box">
                <h2>{"Информация"}</h2>
                <div class="console-log">
                    <p>{"Консоль браузера содержит информацию о действиях и ошибках"}</p>
                </div>
            </div>
        </main>
    }
}
