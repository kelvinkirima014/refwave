import { createSignal } from "solid-js";
import { A } from "solid-start";
import { createServerAction$ } from "solid-start/server";

export default function Login() {

    const [username, setUsername] = createSignal("");

    const handleSubmit = async () => {

        try {
            
            let formData = new URLSearchParams();
            formData.append('username', username());

            let response = await fetch("http://127.0.0.1:8080/users/login", {
                method: 'POST',
                mode: "cors",
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                },
                body: formData,
            });

            if (response.ok) {
                console.log("Login Successful!");
            } else {
                console.error("login failed:", await response.text());
            }

        } catch (error) {
            console.error("Error occured:", error) ;           
        }
    }


    return(
        <main class="min-h-screen bg-gray-900 text-center mx-auto text-gray-300">
            <div class="flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
                <div>
                    <h2 class="text-3xl mb-4">Login Page</h2>
                    <input 
                        onInput={(e) => setUsername(e.target.value)}
                        value={username()}
                        name="username"
                        placeholder="please login with your username"
                        class="text-gray-900 mb-4 p-2 w-full border rounded-md"    
                    />
                    <button
                        onClick={handleSubmit}
                        class="bg-red-200 text-gray-900 mt-8 ml-5 py-2 px-4 rounded-full"
                    >LogIn
                    </button>
                    <p class="mt-4 text-xl py-2">Hello {username()}, Howyu doin? </p>
                </div>
            </div>

            <p>
                <A href="/" class="text-red-200 hover:underline"> Home </A>
                {" - "}
                <span>LogIn Page</span>
            </p>
        </main>
    )

}