import { A } from "solid-start";


export default function Home() {
  return (
    <main class="min-h-screen text-center mx-auto text-gray-300 p-4 bg-gray-900">
      <h1 class="max-6-xs text-6xl text-gray font-thin uppercase my-16">
        Welcome to RefWave!
      </h1>

      <p class="mt-8">
        Please{" "}
        <a
          href="/signup"
          target="_blank"
          class="text-red-200 hover:underline"
        >
          signup{" "}
        </a>
        and refer your friends to join the wave.
      </p>
      <p class="my-4">
        <span>Home</span>
        {" - "}
        <A href="/signup" class="text-red-200 hover:underline">
          Signup Page
        </A>
        {" - "}
        <A href="/referralSignup" class="text-red-200 hover:underline">Refcode Signup</A>
        {" - "}
        <A href="/login" class="text-red-200 hover:underline">LogIn</A>
      </p>
    </main>
  );
}
