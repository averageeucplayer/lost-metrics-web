use yew::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! { <h1>{ "home" }</h1> }
}

    // let app_state = use_state(|| Vec::<String>::new());
    // let encounter_state = use_state(|| None::<Encounter>);

    // {
    //     let app_state = app_state.clone();
    //     spawn_local(async move {
    //         let mut events = event::listen::<String>("app-state").await.unwrap();
    
    //         while let Some(event) = events.next().await {
    //             app_state.set({
    //                 let mut new_state = (*app_state).clone();
    //                 new_state.push(event.payload.clone());
    //                 new_state
    //             });
    //         }  
    //     });

    //     let encounter_state = encounter_state.clone();
    //     spawn_local(async move {
    //         let mut events = event::listen::<Encounter>("encounter-update").await.unwrap();
    
    //         while let Some(event) = events.next().await {
    //             encounter_state.set(Some(event.payload));
    //         }  
    //     });
    // }

    // let emit_event = {
    //     Callback::from(move |_| {
    //         spawn_local(async move {
    //             if let Err(err) = event::emit::<String>("check-update", &"".to_string()).await {
    //                 console::log_1(&format!("Error emitting event: {:?}", err).into());
    //             }
    //         });
    //     })
    // };
