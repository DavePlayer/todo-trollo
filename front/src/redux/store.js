import {configureStore} from "@reduxjs/toolkit";
import userReducer from './reducers/user.js'
import groupsReducer from './reducers/groups.js'
import tasksReducer from "./reducers/tasks.js";

export const store = configureStore ({
    reducer: {
        user: userReducer,
        groups: groupsReducer,
        tasks: tasksReducer,
    }
})