use yew::prelude::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::{RequestBuilder, Headers, Method};

type MutationHandler<U> = Box<dyn Fn(String, Option<U>)>;

#[hook]
pub fn use_mutation<T, U>(
    method: Method,
) -> (UseStateHandle<Rc<bool>>, UseStateHandle<Rc<Option<String>>>, MutationHandler<U>) 
where
    T: 'static + DeserializeOwned,
    U: 'static + Serialize + Clone,
{
    let loading_handler = use_state(|| Rc::new(false));
    let error_handler = use_state(|| Rc::new(None));

    let method = Rc::new(method);

    let mutation_handler: MutationHandler<U> = Box::new({
        let loading_handler = loading_handler.clone();
        let error_handler = error_handler.clone();
        let method = method.clone();

        move |url: String, body: Option<U>| {
            spawn_local({
                let loading_handler = loading_handler.clone();
                let error_handler = error_handler.clone();
                let method = method.clone();

                async move {
                    loading_handler.set(Rc::new(true));
                    let request_body = body.clone().map(|b| serde_json::to_string(&b).unwrap_or_default());
                    
                    let headers = Headers::new();
                    headers.set("Content-Type", "application/json");
                    
                    let request_builder_result = RequestBuilder::new(&url)
                        .method(method.as_ref().clone())
                        .headers(headers)
                        .body(request_body);

                    match request_builder_result {
                        Ok(request) => {
                            match request.send().await {
                                Ok(response) => {
                                    println!("{:?}",response)
                                },
                                Err(err) => {
                                    error_handler.set(Rc::new(Some(format!("Send Error: {:?}", err))));
                                }
                            }
                        },
                        Err(err) => {
                            error_handler.set(Rc::new(Some(format!("Request Build Error: {:?}", err))));
                        }
                    };
                    loading_handler.set(Rc::new(false));
                }
            });
        }
    });
    (loading_handler, error_handler, mutation_handler)
}