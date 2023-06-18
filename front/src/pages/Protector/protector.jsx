import React, { ReactNode, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Navigate, useNavigate } from "react-router-dom";
import { readToken } from "./../../redux/reducers/user";

export const Protector = ({ children }) => {
    const user = useSelector((state) => state.user);
    const [fetched, setFetched] = useState(false);
    const [error, setError] = useState("");
    const dispatch = useDispatch();
    const navigate = useNavigate();
    useEffect(() => {
        //get token from local storage. forward to login if token doesn't exist
        const token = localStorage.getItem("token");
        if (token == null || token == undefined || token == "undefined") {
            localStorage.removeItem("token");
            return navigate("/login");
        }

        if (user.jwt.length <= 0 && token != null && token.length > 0) {
            dispatch(readToken({ token }));
            setFetched(true);
        }

        // secure fake token paste (verify before forwarding to element)
        if (user.jwt.length > 0) {
            setFetched(true);
        }
    }, []);
    console.log("past protector");
    return (
        <>
            {error && error.length > 0 ? (
                <Navigate to={`/login?${new URLSearchParams({ error })}`} />
            ) : (
                fetched &&
                (children ? (
                    children
                ) : (
                    <Navigate
                        to={`/login?${new URLSearchParams({
                            error: "no react component provided",
                        })}`}
                    />
                ))
            )}
        </>
    );
};
