import React, { ReactNode } from "react";

export const Wraper = ({ children, visibilityChange }) => {
    const handleWrapperClick = (event) => {
        // Check if the click event originated from the wrapper box itself (background)
        if (event.target === event.currentTarget) {
            visibilityChange(false);
        }
    };
    return (
        <section className="wrapperBox" onClick={handleWrapperClick}>
            {children}
        </section>
    );
};
