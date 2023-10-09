import { createSignal } from "solid-js";
import { A, useNavigate } from "solid-start";
import { createServerAction$ } from "solid-start/server";


export default function Signup() {
    const navigate = useNavigate()

    const [username, setUsername] = createSignal("");
    
    const handleSubmit = async () => {
        if (username().length < 3) {
            console.log("please add a username with three or more characters");
        } 

        try {

                let formData = new URLSearchParams();
                formData.append('username', username());

                let response = await fetch("http://127.0.0.1:8080/users/signup-username", {
                    method: 'POST',
                    mode: 'cors',
                    headers: {
                        'Content-Type': 'application/x-www-form-urlencoded',
                    },
                    body: formData,
                });

                if (response.ok) {
                    console.log("signup successful!");
                    navigate("/")
                    //window.location.href = "/";
                } else {
                    console.error("Signup failed:", await response.text());
                }
            } catch (error){
                console.error("Error occured:", error);
        }
        
    }


    return (
       <main class="flex flex-col min-h-screen bg-gray-900 text-center mx-auto text-gray-300">

        <div class=" flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
           
                <div>
                    <h2 class="text-3xl mb-4">Signup Page</h2>
                    <input 
                        onInput={(e) => setUsername(e.target.value)} 
                        value={username()} 
                        name="username" 
                        placeholder="please enter your name"
                        class="text-gray-900 mb-4 p-2 w-full border rounded-md" 
                     />
                     {/* {errormessage() && <div class="text-red-500 mb-2">{errormessage()}</div> } */}
                    <button 
                        onClick={handleSubmit}
                        class="bg-red-200 text-gray-900 mt-4 py-2 px-4 rounded-full">
                        Signup
                    </button>
                    <p class="mt-4 text-xl py-2">Hello {username()}, Howyu doin?</p>
                </div>
                
           
        </div>

            <div class="mt-auto m-4">
                <p>
                <A href="/" class="text-red-200 hover:underline">
                Home
                </A>
                {" - "}
                <span>Signup Page</span>
                </p>
            </div>

            {/* <p>
                <A href="/" class="text-red-200 hover:underline">
                Home
                </A>
                {" - "}
                <span>Signup Page</span>
            </p> */}
        </main>
    )
}



