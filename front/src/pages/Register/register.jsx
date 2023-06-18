import React, { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useDispatch } from "react-redux";
import { fetchRegister } from "./../../redux/reducers/user";

export const Register = () => {
    const navigate = useNavigate();
    const dispatch = useDispatch();
    const [form, setForm] = useState({ login: "", password: "", confpassword: "", name: "" });
    const handleClick = (e) => {
        e.preventDefault();
        navigate("/");
        dispatch(fetchRegister(form));
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
    return (
        <div className="formPage">
            <h1>Sign Up</h1>
            <form>
                <label htmlFor="login">
                    <p>Email:</p>
                    <input
                        type="text"
                        name="login"
                        id="login"
                        value={form.login}
                        onChange={(e) => handleChange(e)}
                    />
                </label>
                <label htmlFor="name">
                    <p>Name:</p>
                    <input
                        type="text"
                        name="name"
                        id="name"
                        value={form.name}
                        onChange={(e) => handleChange(e)}
                    />
                </label>
                <label htmlFor="password">
                    <p>Password:</p>
                    <input
                        type="password"
                        name="password"
                        id="password"
                        value={form.password}
                        onChange={(e) => handleChange(e)}
                    />
                </label>
                <label htmlFor="confpassword">
                    <p>Confirm Password:</p>
                    <input
                        type="password"
                        name="confpassword"
                        id="confpassword"
                        value={form.confpassword}
                        onChange={(e) => handleChange(e)}
                    />
                </label>
                <button onClick={(e) => handleClick(e)}>Less' go</button>
            </form>
            <Link to={"/"}>Log in</Link>
        </div>
    );
};
