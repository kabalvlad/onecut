use yew::prelude::*;
use crate::models::{AppState, AppAction};

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
}

#[function_component(MainContent)]
pub fn main_content(props: &Props) -> Html {
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
                                        checked={props.state.cutting_type == "laser"}
                                        onchange={props.handle_cutting_type_change.clone()}
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
                                        checked={props.state.cutting_type == "plasma"}
                                        onchange={props.handle_cutting_type_change.clone()}
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
                                        checked={props.state.cutting_type == "hydro"}
                                        onchange={props.handle_cutting_type_change.clone()}
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
                                        checked={props.state.material == "aluminum"}
                                        onchange={props.handle_material_change.clone()}
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
                                        checked={props.state.material == "steel"}
                                        onchange={props.handle_material_change.clone()}
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
                                        checked={props.state.material == "stainless"}
                                        onchange={props.handle_material_change.clone()}
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
                                        checked={props.state.material == "copper"}
                                        onchange={props.handle_material_change.clone()}
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
                                        checked={props.state.material == "plastic"}
                                        onchange={props.handle_material_change.clone()}
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
                            value={props.state.thickness.clone()}
                            onchange={props.handle_thickness_input.clone()}
                        />
                        <select onchange={props.handle_thickness_select.clone()}>
                            <option value="">{"Выберите толщину"}</option>
                            {
                                props.all_thicknesses.iter().map(|&thickness| {
                                    html! {
                                        <option value={thickness.to_string()} selected={props.state.thickness == thickness.to_string()}>
                                            {format!("{} мм", thickness)}
                                        </option>
                                    }
                                }).collect::<Html>()
                            }
                        </select>
                        <button onclick={props.handle_clear_thickness.clone()} class="clear-button">
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
                                value="file"
                                onclick={
                                    let state = props.state.clone();
                                    Callback::from(move |_| {
                                        state.dispatch(AppAction::AddHistoryMessage("Выбран режим ввода: файл".to_string()));
                                    })
                                }
                            />
                            {"Выбрать файл"}
                        </label>
                        <label>
                            <input 
                                type="radio"
                                name="input-mode"
                                value="manual"
                                onclick={
                                    let state = props.state.clone();
                                    Callback::from(move |_| {
                                        state.dispatch(AppAction::AddHistoryMessage("Выбран режим ввода: ручной".to_string()));
                                    })
                                }
                            />
                            {"Ручной ввод"}
                        </label>
                    </div>
                    <div class="file-input-container">
                        <div class="manual-input-container">
                            <input
                                type="text"
                                placeholder="Введите длину реза в мм"
                                onchange={props.handle_cut_length_input.clone()}
                            />
                        </div>
                        <div class="cut-length-display">
                            <span>{"Длина реза в мм: "}</span>
                            <span>
                                {
                                    if props.state.cut_length > 0.0 {
                                        format!("{:.2} мм", props.state.cut_length)
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
                        
                        <div class="section-column">
                            <h2>{r"Резьба \ вставки \ цековки "}</h2>
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
                    </div>
                </div>
    
                <div class="section-other-options">
                    <div class="options-row">
                        <div class="input-group">
                            <label for="parts-count">{"Количество деталей:"}</label>
                            <input 
                                type="text" 
                                value={props.state.parts_count.to_string()} 
                                onchange={props.handle_parts_count_change.clone()} 
                            />
                        </div>
                        <div class="input-group">
                            <label for="material-price">{"Цена материала за деталь (€):"}</label>
                            <input 
                                type="text"
                                placeholder="Введите цену заготовки"  
                                value={props.state.material_price.to_string()}
                                onchange={props.handle_material_price_change.clone()} 
                            />
                        </div>
                        
                        <div class="input-group">
                            <label for="margin">{"Маржа заказа (%):"}</label>
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
                    <div class="results-row">
                        <div class="result-group">
                            <label>{"Цена за одну деталь (€):"}</label>
                            <div class="result-value" id="price-per-part">
                                {format!("{:.2}", props.state.price_per_part)}
                            </div>
                        </div>                   
                        
                        <div class="result-group">
                            <label>{"Цена за партию деталей (€):"}</label>
                            <div class="result-value" id="price-total">
                                {format!("{:.2}", props.state.price_total)}
                            </div>
                        </div>
                    </div>                    
                </div>
            </div>
            
            <div class="info-box">
                <h2>{"Информация"}</h2>
                <div class="history-list">
                    {
                        props.state.history.iter().map(|message| {
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
