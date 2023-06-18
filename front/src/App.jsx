import { useState } from "react";
import { Routes, Route, Navigate } from "react-router-dom";
import { Login } from "./pages/Login/login";
import { Register } from "./pages/Register/register";
import { Dashboard } from "./pages/Dashboard/dashboard";
import { Protector } from "./pages/Protector/protector";
import { Logout } from "./pages/Logout/logout";
import { Navbar } from "./shared/Navbar/navbar";
import { ToastContainer } from "react-toastify";

// when you fetch data use line adres bellow instead of full address
// import.meta.env.API_URL
// later on changing api url from 127.0.0.1 will be done by changing just one line instead of 1000 lines

function App() {
    const [count, setCount] = useState(0);
    return (
        <>
            <ToastContainer autoClose={2000} closeOnClick={true} pauseOnHover={true} />
            <Routes>
                <Route
                    exact
                    path="/"
                    element={
                        <Protector>
                            <div className="main">
                                <Protector />
                            </div>
                        </Protector>
                    }
                ></Route>
                <Route
                    path="/login"
                    element={
                        <div className="main">
                            <Login />
                        </div>
                    }
                ></Route>
                <Route
                    path="/register"
                    element={
                        <div className="main">
                            <Register />{" "}
                        </div>
                    }
                ></Route>
                <Route path="/logout" element={<Logout />}></Route>
                <Route
                    path="/dashboard"
                    element={
                        <Protector>
                            <>
                                <Navbar />
                                <Dashboard />
                            </>
                        </Protector>
                    }
                ></Route>
            </Routes>
        </>
    );
}

export default App;
