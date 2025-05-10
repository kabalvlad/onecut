
use yew::prelude::*;
use crate::components::MainContent;

#[function_component(App)]
pub fn app() -> Html {

    // Определяем все доступные толщины материала
    let all_thicknesses: Vec<f32> = vec![
        0.5, 0.7, 0.8, 1.0, 1.2, 1.5, 2.0, 2.5, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0, 10.0, 12.0, 14.0, 15.0, 16.0, 
        20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0
    ];
    
    // Клонируем вектор для использования в JSX
    let all_thicknesses_for_jsx = all_thicknesses.clone();

    html! {
        <MainContent
            all_thicknesses={all_thicknesses_for_jsx}
            
        />
    }  
}
