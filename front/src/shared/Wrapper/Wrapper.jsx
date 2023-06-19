import React, { ReactNode } from "react";

export const Wraper = ({ children, visibilityChange }) => {
    return <section className="wrapperBox">{children}</section>;
};
