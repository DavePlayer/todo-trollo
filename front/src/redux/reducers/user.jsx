import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";

const initialState = {
    loading: true,
    id: "",
    email: "",
    isLoggedIn: false,
    jwt: ""
};

export const fetchLogin = createAsyncThunk(
    '/login',
    ({ login, password }) => {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                const test = Math.floor(Math.random() * 10);
                if (test < 5) {
                    resolve({
                        id: "0",
                        email: "basic@example.com",
                        jwt: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MCwicGFzc3dvcmQiOiJwYXNzd29yZCIsImxvZ2luIjoiYmFzaWNAZXhhbXBsZS5jb20ifQ.fjQKIH0zywmQYEpyPtNpQyRo9h4AImLZVs_obHCkzkQ"
                    });
                } else {
                    reject('Failed to fetch login')
                }
            },1000)
        })
    }
);

export const userSlice = createSlice ({
    name: "user",
    initialState,
    reducers: {
        logout: (state) => {
            console.log('logging out...');
            return(state = {
                id: "",
                email: "",
                isLoggedIn: false,
                jwt: ""
            });
        }
    },
    extraReducers: (builder) => {
        builder.addCase(fetchLogin.pending, (state) => {
            state.loading = true;
        });
        builder.addCase(fetchLogin.fulfilled, (state, action) => {
            state.loading = false;
            state.isLoggedIn = true;
            state.id = action.payload.id;
            state.email = action.payload.email;
            state.jwt = action.payload.jwt;
        });
        builder.addCase(fetchLogin.rejected, (state, action) => {
            console.error(action.error.message);
        });
    },
});

export const { logout } = userSlice.actions;

export default userSlice.reducer;