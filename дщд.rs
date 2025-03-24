let bending_radio_state = use_state(|| "no");
let bending_value = use_state(|| String::new());

let handle_bending_radio = Callback::from(move |event: Event| {
    let target = event.target();
    let value = target.unwrap().value_of().as_string().unwrap();
    bending_radio_state.set(value);
});

html! {
    <div class="block">
        <span>{"Места гибки"}</span>
        <label>
            <input type="radio" name="bending" value="no" checked={*bending_radio_state == "no"} onchange={handle_bending_radio.clone()} />
            {"Нет"}
        </label>
        <label>
            <input type="radio" name="bending" value="yes" checked={*bending_radio_state == "yes"} onchange={handle_bending_radio.clone()} />
            {"Да"}
        </label>
        <input type="number" value={(*bending_value).clone()} disabled={*bending_radio_state == "no"} onchange={handle_bending_input.clone()} />
    </div>
}
