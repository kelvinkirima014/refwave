import { A } from "solid-start";
import { createSignal, onCleanup, onMount } from "solid-js";

type DashboardItem = {
  username: string;
  id: number;
  referral_code: string;
  referred_by: string;
  invited_users_count: number;
  created_at: string;
  updated_at: string;
};


export default function Home() {

  const [username, setUsername] = createSignal("");
  const [loggedIn, setLoggedIn] = createSignal(false);
  const [data, setData] = createSignal<DashboardItem[]>([]);
  const [searchData, setSearchData] = createSignal<DashboardItem[]>([]);
  const[searchUsername, setSearchUsername] = createSignal("");

  
  const fetchData = async () => {
    console.log("Fetching data...");
    const token = localStorage.getItem('jwt');
    try {
      const response = await fetch("http://127.0.0.1:8080/users/view", {
        method: 'GET',
        mode: 'cors',
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });

      if (response.ok) {
        const jsonData = await response.json();
        setSearchData(jsonData);
        setData(jsonData);
      } else {
        console.error("failed to fetch data:", await response.text());
      }

    } catch (error) {
      console.error("Error occured:", error);      
    }
  }

  const setupSSE = () => {
    let eventSource = new EventSource("http://127.0.0.1:8080/sse");

    eventSource.onmessage = (event) => {
      const updateData = JSON.parse(event.data);
      setData(updateData);
    };

    eventSource.onerror = (error) => {
      console.error("EventSource Failed:", error);
      eventSource.close();
    }
  }

  const initializeDashboard = async () => {
    await fetchData();
    setupSSE();
  }

  //We need this here to handle the logIn logic
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
                const responseData = await response.json();
                localStorage.setItem('jwt', responseData.token);
                localStorage.setItem('username', username())
                console.log("Login Successful!");
                setLoggedIn(true);
                initializeDashboard();
            } else {
                console.error("login failed:", await response.text());
            }

        } catch (error) {
            console.error("Error occured:", error) ;           
        }
    }



  
  onMount(() => {
    const token = localStorage.getItem('jwt');
    const storedUsername = localStorage.getItem('username') || 'Guest';//default to guest if null
    if (token && storedUsername) {
      setUsername(storedUsername);
      setLoggedIn(true);
      initializeDashboard();
    }
  });

 
  const renderNotLoggedInContainer = () => (
    <div class="min-h-screen text-center mx-auto text-gray-300 p-4 bg-gray-900">
      
       <h1 class="max-6-xs text-6xl text-gray font-thin uppercase my-16">
        Welcome to RefWave!
      </h1>

      
      <div class="mt-8 flex flex-col items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div class="w-64"> {/* Set a fixed width for the container */}
        <input
          onInput={(e) => setUsername(e.target.value)}
          value={username()}
          name="username"
          placeholder="please login with your username"
          class="text-gray-900 mb-4 p-2 w-full border rounded-md"
        />
        <button
          onClick={handleSubmit}
          class="bg-red-200 text-gray-900 mt-4 py-2 px-4 rounded-full"
        >
          LogIn
        </button>
      </div>
      <p class="m-4">Or</p>
     <p class=""><A href="/signup" class="text-white hover:underline">Register an Account</A></p>
    </div>

    </div>
  );

  // Render the dashboard data in a table
  const renderDashboard = () => {
    console.log("Rendering dashboard", data());
    return (
      <div class="flex justify-center items-center">
        <table class="table-auto mt-8">
        <thead>
          <tr>
            <th class="px-4 py-2">Username</th>
            <th class="px-4 py-2">ID</th>
            <th class="px-4 py-2">Referral Code</th>
            <th class="px-4 py-2">Referred By</th>
            <th class="px-4 py-2">Invited Users Count</th>
          </tr>
        </thead>
        <tbody>
          {data().map((item) => (
            <tr>
              <td class="px-4 py-2">{item.username}</td>
              <td class="px-4 py-2">{item.id}</td>
              <td class="px-4 py-2">{item.referral_code}</td>
              <td class="px-4 py-2">{item.referred_by}</td>
              <td class="px-4 py-2">{item.invited_users_count}</td>
            </tr>
          ))}
        </tbody>
      </table>

      </div>
     
    );
  };

  const handleSearch = (value: string) => {
    setSearchUsername(value);
    if(searchUsername().trim() == "") {
      setData(searchData());
    } else {
      const filteredData = searchData().filter(item => item.username.toLocaleLowerCase()
      .includes(searchUsername().toLowerCase()));
      setData(filteredData);
    }
  }

  const handleLogout = async() => {
    localStorage.removeItem('jwt');
    localStorage.removeItem('username');
    setLoggedIn(false);
  }

  
  const renderLoggedInContainer = () => (
    <div class="relative min-h-screen text-center mx-auto text-gray-300 p-4 bg-gray-900">
       <h1 class="max-6-xs text-6xl text-gray font-thin uppercase my-16">
        Welcome Back, {username()}!
      </h1>

      <div class="mb-4">
        <input
          onInput={(e) => handleSearch(e.target.value)}
          value={searchUsername()}
          placeholder="Search for a username..."
          class="text-gray-900 p-2 w-2/3 border rounded-md"
        />
      </div>
      {renderDashboard()}

      <button 
        onClick={handleLogout} 
        class="absolute top-[-42px] right-4 bg-red-200 text-gray-900 py-2 px-4 rounded-full"
        >Logout
      </button>
    </div>
  )


  return (
    <div class="min-h-screen text-center mx-auto text-gray-300 p-4 bg-gray-900">
      {!loggedIn() ? renderNotLoggedInContainer() : renderLoggedInContainer()}
    </div>
  );
}