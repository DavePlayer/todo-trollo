import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import jwtDecode from "jwt-decode";

const initialState = {
    loading: true,
    id: "",
    email: "",
    name: "",
    isLoggedIn: false,
    jwt: "",
};

export const fetchLogin = createAsyncThunk("/login", ({ login, password }) => {
    const resolve = fetch(`http://127.0.0.1:8080/auth/login/${login}/${password}`, {
        method: "GET",
    }).then(async (data) => {
        if (!data.ok) {
            throw new Error(`${data.status}: ${await data.text()}`);
        }
        return data.json();
    });
    toast.promise(resolve, {
        pending: {
            render() {
                return "I'm loading";
            },
            type: "pending",
        },
        success: {
            render({ data }) {
                return `Logged in syccessfully`;
            },
            // other options
        },
        error: {
            render({ data }) {
                // When the promise reject, data will contains the error
                return data.message;
            },
            type: "error",
        },
    });
    return resolve;
});

export const fetchRegister = createAsyncThunk("/register", (data) => {
    const body = {
        name: data.name,
        login: data.login,
        password: data.password,
    };
    console.log(JSON.stringify(body));
    const resolve = fetch(`http://127.0.0.1:8080/auth/register`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
    }).then(async (data) => {
        if (!data.ok) {
            console.log(data);
            throw new Error(`${data.status}: ${await data.text()}`);
        }
        return data.json();
    });
    toast.promise(resolve, {
        pending: {
            render() {
                return "I'm loading";
            },
            type: "pending",
        },
        success: {
            render({ data }) {
                return `Registered successfully`;
            },
            // other options
        },
        error: {
            render({ data }) {
                // When the promise reject, data will contains the error
                return data.message;
            },
            type: "error",
        },
    });
    return resolve;
});

export const userSlice = createSlice({
    name: "user",
    initialState,
    reducers: {
        logout: (state) => {
            console.log("logging out...");
            localStorage.removeItem("token");
            return (state = {
                id: "",
                email: "",
                isLoggedIn: false,
                jwt: "",
                name: "",
            });
        },
        readToken: (state, action) => {
            const { token } = action.payload;
            console.log("validating token: ", token);
            const data = jwtDecode(token);
            state.id = data.id;
            state.jwt = token;
            state.name = data.name;
            state.email = data.email;
            state.isLoggedIn = data.true;
        },
    },
    extraReducers: (builder) => {
        builder.addCase(fetchLogin.fulfilled, (state, action) => {
            state.loading = false;
            state.isLoggedIn = true;
            state.id = action.payload.id;
            state.email = action.payload.login;
            state.name = action.payload.name;
            state.jwt = action.payload.token;

            localStorage.setItem("token", action.payload.token);
        });
        builder.addCase(fetchLogin.rejected, (state, action) => {
            console.error(action.error.message);
            // toast.update(id, { render: action.error.message, type: "error" });
        });

        builder.addCase(fetchRegister.fulfilled, (state, action) => {
            state.loading = false;
            state.isLoggedIn = true;
            state.id = action.payload.id;
            state.email = action.payload.login;
            state.name = action.payload.name;
            state.jwt = action.payload.token;

            localStorage.setItem("token", action.payload.token);
        });
        builder.addCase(fetchRegister.rejected, (state, action) => {
            console.error(action.error.message);
            // toast.update(id, { render: action.error.message, type: "error" });
        });
    },
});

export const { logout } = userSlice.actions;
export const { readToken } = userSlice.actions;

export default userSlice.reducer;
