import React, { ReactNode, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Navigate, useNavigate } from "react-router-dom";

export const Protector = ( {children} ) => {
    const user = useSelector((state) => state.user);
    const [error, setError] = useState("");
    const dispatch = useDispatch();
    const navigate = useNavigate();
    useEffect(() => {
        if(user.jwt.length <= 0){
            return navigate("/login");
        }
        return;
    });
    console.log('past protector')
    return(
        <>
            {error.length > 0 ?(
                <Navigate to={`/login?${new URLSearchParams({ error })}`} />
            ) : (
                (children ? (
                    children
                ): (
                    <Navigate to={`/login?${new URLSearchParams({error: "no react component provided",})}`}/>
                ))
            )}
        </>
    );
}