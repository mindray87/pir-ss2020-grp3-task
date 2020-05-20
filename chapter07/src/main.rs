
mod api;
mod db;
mod util;


fn main () {

    // first you'll need to create the module structure to make the following
    // calls possible

    // after your project is setup properly, we want to establish a connection
    // therefore in the util module you'll need to provide a struct Connection
    // that holds an ip as a String
    let connection = api::users::Connection{
        ip: "111.222.333.444:55".to_string(),
    };

    // now make the connection -> this should print that the connection is working.
    connection.connect();

    // now make use of the user module you created, to construct a new user. 
    // each user will have the fields name, age and city
    let user = api::users::User::new("Julian".to_string(), 24, "Konstanz".to_string());

    // save the user in a db -> this should print the new user entry with its fields
    db::save_user(...);

    // now we'll let the user make a post in an imaginary forum
    // if successful this should print the user's name and age as well as the message

    api::posts::make_post(...)
    
    // after your first make_post() is working, rebuild it while using
    // the new struct Message you will create. Assign a random generated id
    // to each message (6 digits&|chars long). the new call to make_posts
    // should print the username, age, city, the message and the id.
    // hint: there's a rand crate you all should know about, if you've read the book
    // it has a message called sample_iter, which can produce alphanumerics.
    // let message = api::posts::Message{
    //     message: "Hello".to_string(),
    //     user: user.name,
    //     id: util::make_id()
    // };

    // message.make_post();
}