import React, { useState } from "react";
import { Link, useNavigate } from "react-router-dom";

export const Login = () => {
    const navigate = useNavigate();
    const [form, setForm] = useState({login:"", password:""});
    const handleClick = (e) => {
        e.preventDefault();
        navigate("/dashboard");
    };
    const handleChange = (e) => {
        let { name, value } = e.target;
        setForm((prev) => {
            return {
                ...prev,
                [name]: value,
            };
        });
    };
    return(
        <div className="formPage" >
            <h1>User Login</h1>
            <form>
                <label htmlFor="login">
                    <p>Email:</p>
                    <input type="text" name="login" id="login" value={form.login} onChange={(e) => handleChange (e)}/>
                </label>
                <label htmlFor="password">
                    <p>Password:</p>
                    <input type="password" name="password" id="password" value={form.password} onChange={(e) => handleChange (e)}/>
                </label>
                <button onClick={(e) => handleClick(e)}>Less' go</button>
            </form>
            <Link to={'/register'}>Sign up</Link>
        </div>
    );
};
