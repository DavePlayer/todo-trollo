import { createSlice } from "@reduxjs/toolkit";

const initialState = {
    id: "",
    email: "basic@example.com",
    isLoggedIn: false,
    jwt: "x"
}

export const userSlice = createSlice ({
    name: "user",
    initialState,
    reducers: {
        logout: (state) => {
            console.log('logging out...');
            return(state = {
                id: "",
                email: "basic@example.com",
                isLoggedIn: false,
                jwt: ""
            });
        }
    },
})

export const { logout } = userSlice.actions;

export default userSlice.reducer;