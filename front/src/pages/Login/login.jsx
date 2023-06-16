import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Link, Navigate, useNavigate } from "react-router-dom";
import { fetchLogin } from "../../redux/reducers/user";

export const Login = () => {
    const navigate = useNavigate();
    const dispatch = useDispatch();
    const user = useSelector((state) => state.user);
    const [form, setForm] = useState({ login: "", password: "" });
    const handleClick = (e) => {
        e.preventDefault();
        console.log(form)
        if (form.login.length <= 0 || form.password.length <= 0)
            return console.error("Something is not yes");
        dispatch(fetchLogin(form));
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
    useEffect(() => {
        if (user.jwt.length > 0) navigate("/dashboard");
    }, []);
    return (
        <>
            {user.isLoggedIn ? (<Navigate to="/dashboard" />) : 
            (
                <div className="formPage" >
                    <h1>User Login</h1>
                    <form>
                        <label htmlFor="login">
                            <p>Email:</p>
                            <input type="text" name="login" id="login" value={form.login} onChange={(e) => handleChange(e)} />
                        </label>
                        <label htmlFor="password">
                            <p>Password:</p>
                            <input type="password" name="password" id="password" value={form.password} onChange={(e) => handleChange(e)} />
                        </label>
                        <button onClick={(e) => handleClick(e)}>Less' go</button>
                    </form>
                    <Link to={'/register'}>Sign up</Link>
                </div>
            )}
        </>
    );
};
