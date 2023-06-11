import React, { useEffect } from "react";
import { useDispatch } from "react-redux";
import { Navigate } from "react-router-dom";
import { logout } from "../../redux/reducers/user";

export const Logout = () => {
    const dispatch = useDispatch();
    useEffect(() => {
        dispatch(logout());
        return;
    });
    return <Navigate to="/login" />;
};