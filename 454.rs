
use yew::prelude::*;
use crate::components::MainContent;
use crate::handlers::{
    handle_thickness_select, handle_material_change, handle_cutting_type_change,
    handle_thickness_input, handle_clear_thickness, handle_input_mode_change, 
    handle_cut_length_input, handle_file_select, handle_bending_points_radio_change,
    handle_bending_points_input_change, handle_threads_inserts_mats_radio_change,
    handle_threads_inserts_mats_input_change, handle_parts_count_change,
    handle_material_price_change, handle_margin_change, handle_clear_options,
};

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

    let handle_cutting_type_change = {
        Callback::from(move |e: Event| {
            handle_cutting_type_change().emit(e);
        })
    };
    
    let handle_material_change = {
        Callback::from(move |e: Event| {
            handle_material_change().emit(e);
        })
    };

    let handle_thickness_select = {
        Callback::from(move |e: Event| {
            handle_thickness_select().emit(e);
        })
    };
    
    let handle_thickness_input = {
        let thicknesses = all_thicknesses.clone(); 
        Callback::from(move |e: Event| {
            handle_thickness_input(thicknesses.clone()).emit(e);
        })
    };

    let handle_clear_thickness = {
        Callback::from(move |e: MouseEvent| {
            handle_clear_thickness().emit(e);
        })
    };
    
    
    let handle_input_mode_change = {
        Callback::from(move |mode: String| {  
            handle_input_mode_change().emit(mode);
        })
    };  
    
    let handle_cut_length_input = {
        Callback::from(move |e: Event| {
            handle_cut_length_input().emit(e);
        })
    };

    let handle_file_select = {
        Callback::from(move |e: Event| {
            handle_file_select().emit(e);
        })
    }; 

    
    let handle_bending_points_radio_change = {
        Callback::from(move |e: Event| {
            handle_bending_points_radio_change().emit(e);
        })
    };
    
    let handle_bending_points_input_change = {
        Callback::from(move |e: Event| {
            handle_bending_points_input_change().emit(e);
        })
    };
    
    let handle_threads_inserts_mats_radio_change = {
        Callback::from(move |e: Event| {
            handle_threads_inserts_mats_radio_change().emit(e);
        })
    };
    
    let handle_threads_inserts_mats_input_change = {

        Callback::from(move |e: Event| {
            handle_threads_inserts_mats_input_change().emit(e);
        })
    };
    
    let handle_parts_count_change = {
        Callback::from(move |e: Event| {
            handle_parts_count_change().emit(e);
        })
    };
    
    let handle_material_price_change = {
        Callback::from(move |e: Event| {
            handle_material_price_change().emit(e);
        })
    };
    
    let handle_margin_change = {
        Callback::from(move |e: Event| {
            handle_margin_change().emit(e);
        })
    };    
    
    let handle_clear_options = {
        Callback::from(move |e: MouseEvent| {
            handle_clear_options().emit(e);
        })
    };


    html! {
        <MainContent
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
