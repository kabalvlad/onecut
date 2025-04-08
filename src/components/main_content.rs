use yew::prelude::*;
use crate::models::AppState;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub state: UseReducerHandle<AppState>,
    pub all_thicknesses: Vec<f32>,
    pub handle_cutting_type_change: Callback<Event>,
    pub handle_material_change: Callback<Event>,
    pub handle_thickness_input: Callback<Event>,
    pub handle_thickness_select: Callback<Event>,    
    pub handle_clear_thickness: Callback<MouseEvent>,
    pub handle_cut_length_input: Callback<Event>,
    pub handle_bending_points_radio_change: Callback<Event>,
    pub handle_bending_points_input_change: Callback<Event>,
    pub handle_threads_inserts_mats_radio_change: Callback<Event>,
    pub handle_threads_inserts_mats_input_change: Callback<Event>,
    pub handle_parts_count_change: Callback<Event>,
    pub handle_material_price_change: Callback<Event>,
    pub handle_margin_change: Callback<Event>,
    pub handle_clear_options: Callback<MouseEvent>,
    pub handle_input_mode_change: Callback<String>,
    pub handle_file_select    : Callback<Event>,
}

#[function_component(MainContent)]
pub fn main_content(props: &Props) -> Html {
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
                                            checked={props.state.cutting_type == "laser"}
                                            onchange={props.handle_cutting_type_change.clone()}
                                        />
                                        {"Лазерная резка"}
                                    </label>
                                </div>
                                <div class="cutting-type">
                                    <label>
                                        <input type="radio" name="cutting" value="plasma"
                                            checked={props.state.cutting_type == "plasma"}
                                            onchange={props.handle_cutting_type_change.clone()}
                                        />
                                        {"Плазменная резка"}
                                    </label>
                                </div>
                                <div class="cutting-type">
                                    <label>
                                        <input type="radio" name="cutting" value="hydro"
                                            checked={props.state.cutting_type == "hydro"}
                                            onchange={props.handle_cutting_type_change.clone()}
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
                                            checked={props.state.material == "aluminum"}
                                            onchange={props.handle_material_change.clone()}
                                        />
                                        {"Алюминий"}
                                    </label>
                                </div>
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="steel"
                                            checked={props.state.material == "steel"}
                                            onchange={props.handle_material_change.clone()}
                                        />
                                        {"Сталь"}
                                    </label>
                                </div>
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="stainless"
                                            checked={props.state.material == "stainless"}
                                            onchange={props.handle_material_change.clone()}
                                        />
                                        {"Нержавеющая сталь"}
                                    </label>
                                </div>
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="copper"
                                            checked={props.state.material == "copper"}
                                            onchange={props.handle_material_change.clone()}
                                        />
                                        {"Латунь/Бронза/Медь"}
                                    </label>
                                </div>
                                <div class="material-type">
                                    <label>
                                        <input type="radio" name="material" value="plastic"
                                            checked={props.state.material == "plastic"}
                                            onchange={props.handle_material_change.clone()}
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
                                    value={props.state.thickness.clone()}
                                    onchange={props.handle_thickness_input.clone()}
                                />
                                <select onchange={props.handle_thickness_select.clone()}>
                                    <option value="">{"Выберите толщину"}</option>
                                    {props.all_thicknesses.iter().map(|&thickness| {
                                        html! {
                                            <option value={thickness.to_string()} 
                                                selected={props.state.thickness == thickness.to_string()}>
                                                {format!("{} мм", thickness)}
                                            </option>
                                        }
                                    }).collect::<Html>()}
                                </select>
                                <button onclick={props.handle_clear_thickness.clone()} class="clear-button">
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
                                        checked={props.state.is_file_selected}
                                        onclick={
                                            let handle_mode_change = props.handle_input_mode_change.clone();
                                            Callback::from(move |_| handle_mode_change.emit("file".to_string()))
                                        }
                                    />
                                    {"Выбрать файл"}
                                </label>
                                <label>
                                    <input type="radio" name="input-mode" value="manual"
                                        checked={props.state.is_manual_input}
                                        onclick={
                                            let handle_mode_change = props.handle_input_mode_change.clone();
                                            Callback::from(move |_| handle_mode_change.emit("manual".to_string()))
                                        }
                                    />
                                    {"Ручной ввод"}
                                </label>
                            </div>
                            <div class="file-input-container">
                                {if props.state.is_file_selected {
                                    html! {
                                        <div class="file-input">
                                            <input type="file" id="dxf-file" accept=".dxf"
                                                onchange={props.handle_file_select.clone()}
                                            />
                                            <label for="dxf-file">{""}</label>
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <div class="manual-input-container">
                                            <input type="text" placeholder="Введите длину реза в мм"
                                                onchange={props.handle_cut_length_input.clone()}
                                            />
                                        </div>
                                    }
                                }}
                                <div class="cut-length-display">
                                    <span>{"Длинна реза
                                     в мм: "}</span>
                                    <span>
                                        {if props.state.cut_length > 0.0 {
                                            format!("{:.2} мм", props.state.cut_length)
                                        } else {
                                            "пусто".to_string()
                                        }}
                                    </span>
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
                                checked={!props.state.bending_points_enabled}
                                onchange={props.handle_bending_points_radio_change.clone()}
                            />
                            {"Нет"}
                        </label>
                        <label>
                            <input 
                                type="radio"
                                name="bending-points"
                                value="yes"
                                checked={props.state.bending_points_enabled}
                                onchange={props.handle_bending_points_radio_change.clone()}
                            />
                            {"Да"}
                        </label>
                    </div>
                    <div class="number-input-container">
                        <input 
                            type="number"
                            placeholder="Количество"
                            min="0"
                            value={props.state.bending_points_count.map_or("".to_string(), |v| v.to_string())}
                            disabled={!props.state.bending_points_enabled}
                            onchange={props.handle_bending_points_input_change.clone()}
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
                                checked={!props.state.threads_inserts_mats_enabled}
                                onchange={props.handle_threads_inserts_mats_radio_change.clone()}
                            />
                            {"Нет"}
                        </label>
                        <label>
                            <input 
                                type="radio"
                                name="threads-inserts-mats"
                                value="yes"
                                checked={props.state.threads_inserts_mats_enabled}
                                onchange={props.handle_threads_inserts_mats_radio_change.clone()}
                            />
                            {"Да"}
                        </label>
                    </div>
                    <div class="number-input-container">
                        <input 
                            type="number"
                            placeholder="Количество"
                            min="0"
                            value={props.state.threads_inserts_mats_count.map_or("".to_string(), |v| v.to_string())}
                            disabled={!props.state.threads_inserts_mats_enabled}
                            onchange={props.handle_threads_inserts_mats_input_change.clone()}
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
                                value={props.state.parts_count.to_string()} 
                                onchange={props.handle_parts_count_change.clone()} 
                            />
                        </div>
                        <div class="input-group">
                            <label for="material-price">{"Стоимость (€):"}</label>
                            <input 
                                type="text"
                                placeholder="Цена"  
                                value={props.state.material_price.to_string()}
                                onchange={props.handle_material_price_change.clone()} 
                            />
                        </div>
                    </div>
                    // Вторая строка: маржа и кнопка очистить
                    <div class="options-row">
                        <div class="input-group">
                            <label for="margin">{"Маржа (%):"}</label>
                            <input 
                                type="text" 
                                value={props.state.margin.to_string()}
                                onchange={props.handle_margin_change.clone()} 
                            />
                        </div>
                        <button 
                            class="clear-button" 
                            onclick={props.handle_clear_options.clone()}
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
                            {format!("{:.2}", props.state.price_per_part)}
                        </div>
                    </div>
                    <div class="result-group">
                        <label>{"Цена за партию (€):"}</label>
                        <div class="result-value" id="price-total">
                            {format!("{:.2}", props.state.price_total)}
                        </div>
                    </div>
                </div>
            </div>
            </div>

            

            // Информационный блок
            <div class="info-box">
                <h2>{"Информация"}</h2>
                <div class="history-list">
                    {if props.state.history.is_empty() {
                        html! { <p class="empty-history">{"История пуста"}</p> }
                    } else {
                        props.state.history.iter().map(|message| {
                            html! {
                                <p key={message.clone()}>{message}</p>
                            }
                        }).collect::<Html>()
                    }}
                </div>
            </div>
        </main>
    }
}
