import { createSignal } from "solid-js";
import { A, useNavigate } from "solid-start";
import { createServerAction$ } from "solid-start/server";

export default function ReferralSignup() {
    const navigate = useNavigate();

    const [referralCode, setReferralCode] = createSignal("");
    const [newUsername, setNewUsername] = createSignal("");

    const handleSubmit = async () => {

        try {

            let formData = new URLSearchParams();
            formData.append('referral_code', referralCode());

            let response = await fetch("http://127.0.0.1:8080/users/signup-refcode", {
                method: 'POST',
                mode: 'cors',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                },
                body: formData,
            });

            if (response.ok) {
                const newUser = await response.json();
                setNewUsername(newUser.username);
                console.log("signup successful!", newUser);
                setTimeout(() => navigate("/"), 8000);  //navigate("/")
            } else {
                console.error("signup failed:", await response.text());
            }
            
        } catch (error) {
            console.error("An error occured:", error);
        }
    }


    return(

        <main class="flex flex-col min-h-screen bg-gray-900 text-center mx-auto text-gray-300">

        <div class="flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">

                <div>
                    <h2 class="text-3xl mb-4">Signup with Referral Code</h2>
                    <input 
                        onInput={(e) => setReferralCode(e.target.value)}
                        value={referralCode()}
                        name="referral_code"
                        placeholder="please enter a valid referral code" 
                        class="text-gray-900 mb-4 p-2 w-full border rounded-md"   
                    />
                    <button
                        onClick={handleSubmit}
                        class="bg-red-200 text-gray-900 mt-8 ml-5 py-2 px-4 rounded-full"
                    >Refcode Signup
                    </button>
                    { /* Display the new username if available */ }
                    {
                        newUsername() && (
                            <p class="text-green-200 mt-4">Successfully signed up with username: {newUsername()}, you can now use the username to login</p>
                        )
                    }
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
                <A href='/' class='text-red-200 hover:underline'>
                    Home
                </A>
                {" - "}
                <span>Signup Page</span>
            </p> */}

        </main>

    )


}