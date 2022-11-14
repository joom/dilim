use crate::*;
use yew::{html, Html};

fn dilim() -> Html {
    html! {
        <span class="dilim">
            {"dilim"} 
        </span>
    }
}


macro_rules! letter {
    ($l:expr) => { Cmd::Text(String::from($l)) }
}

pub fn help() -> Html {
    html! {
        <div class="help">
            <p>
                {"This is "} 
                { dilim() }
                {", a structural editor for a simple functional programming language, with Vim-like shortcuts and commands."}
            </p>
            <p>
                {"Our programs consist of a sequence of statements, where each statement is a definition or a term statement. A term can be a lambda, an application or a variable."}
            </p>
            <p>
                {"Incomplete programs are ones that have holes in them. There are different actions you can perform on holes and existing programs, statements, terms, and names."}
            </p>
            <p>
                {"Here is an example development process of a simple program:"}
            </p>
            <p>
                <ol>
                    <li>
                        {"Select the statement hole in the empty program either by clicking or pressing "}
                        {Cmds(vec![letter!("g"), letter!("0"), letter!("h")]).view()}
                        {" in a sequence, which will run the command "}
                        {Cmds(vec![Cmd::Go, Cmd::Number(0), Cmd::Hole]).view()}
                        {"."}
                        <br/>
                        <br/>
                        {"The first letters of the keys you press correspond to the commands you give. For future directions, you will only see the command, not the keys to press. (Although the keys to press are also bolded and underlined.)"}
                    </li>

                    <li>
                        {"Let's add a definition statement first. Run "}
                        {Cmds(vec![Cmd::Add, Cmd::Defn]).view()}
                        {"."}
                    </li>
                    <li>
                        {"Now hole #1 should be selected. This is where you add a term. Run "}
                        {Cmds(vec![Cmd::Number(2), Cmd::Add, Cmd::Lam]).view()}
                        {". This will add 2 nested lambda terms."}
                    </li>
                    <li>
                        {"Now hole #3, the body of nested lambdas, should be selected. This is where you add the function body. Run "}
                        {Cmds(vec![Cmd::Add, Cmd::App]).view()}
                        {". This will add a function application."}
                    </li>
                    <li>
                        {"Now hole #3 should still be selected. You can add a variable term by running "}
                        {Cmds(vec![Cmd::Add, Cmd::Var]).view()}
                        {", which will create a variable term and a name hole."}
                    </li>
                    <li>
                        {"Now the name hole should be selected. You can put in a variable name by running "}
                        {Cmds(vec![Cmd::Insert, letter!("f"), Cmd::Enter]).view()}
                        {". By now you must have gotten the hang of creating terms in the editor."}
                    </li>
                    <li>
                        {"You can select the entire statement now by running "}
                        {Cmds(vec![Cmd::Number(5), Cmd::Go, Cmd::Outwards]).view()}
                        {". This command selects the immediate outer command 5 times. Feel free to experiment by changing the number, or omitting it, or trying the command "}
                        {Cmds(vec![Cmd::Go, Cmd::Inwards]).view()}
                        {"."}
                        <br/> <br/>
                        {"There are also other commands to move between entities, such as "}
                        {Cmds(vec![Cmd::Go, Cmd::Prev, Cmd::Sibling]).view()}
                        {", "}
                        {Cmds(vec![Cmd::Go, Cmd::Next, Cmd::Sibling]).view()}
                        {", "}
                        {Cmds(vec![Cmd::Go, Cmd::Prev, Cmd::Hole]).view()}
                        {", and "}
                        {Cmds(vec![Cmd::Go, Cmd::Next, Cmd::Hole]).view()}
                        {"."}
                    </li>
                    <li>
                        {"You can select a name hole, either by clicking or with the movement commands, and then run "}
                        {Cmds(vec![Cmd::Add, Cmd::Fresh]).view()}
                        {", which will add a fresh variable name, i.e. a name that hasn't appeared anywhere else in the program."}
                    </li>
                    <li>
                        {"You can cut a part of the program by running "}
                        {Cmds(vec![Cmd::Delete]).view()}
                        {", copy by running "}
                        {Cmds(vec![Cmd::Yank]).view()}
                        {", or paste by running "}
                        {Cmds(vec![Cmd::Paste]).view()}
                        {". This is all similar to Vim."}
                    </li>
                    <li>
                        {"If you want to add an entity surrounding another entity, you can use the "}
                        <kbd class="usual">{"Wrap"}</kbd>
                        {" command. For example, you can select the application term we constructed above, and then run "}
                        <span class="kbdsequence">
                            <kbd class="usual">{"Wrap"}</kbd>
                            <kbd class="usual">{"Lam"}</kbd>
                        </span>
                        {", which would add one more nested lambda just outside the application term."}
                    </li>
                </ol>
            </p>
            <p>
                {"Following the Vim tradition, we intend "}
                { dilim() }
                {" commands to compose as much as they can, as long as it makes sense. Try compositions that make sense to you. You can use "}
                {Cmds(vec![letter!("Tab")]).view()}
                {" what commands are available, even at the middle of a command! If the command works, great! If it doesn't work, let us know and we'll fix it or add it to the editor! Happy hacking!"}
            </p>
        </div>
    }
}
