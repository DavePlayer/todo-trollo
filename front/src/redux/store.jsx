import {configureStore} from "@reduxjs/toolkit";
import userReducer from './reducers/user.jsx'
import groupsReducer from './reducers/groups.jsx'

export const store = configureStore ({
    reducer: {
        user: userReducer,
        groups: groupsReducer,
    }
})