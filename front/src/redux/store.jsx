import {configureStore} from "@reduxjs/toolkit";
import userReducer from './reducers/user.jsx'

export const store = configureStore ({
    reducer: {
        user: userReducer,
    }
})