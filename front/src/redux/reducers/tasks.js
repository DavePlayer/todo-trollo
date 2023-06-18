import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

const initialState = {
    loading: false,
    data: [
        {
            id: 0,
            name: "Task 1",
            crossed_by_id: null,
            group_id: 1,
        },
        {
            id: 1,
            name: "Task 2",
            crossed_by_id: null,
            group_id: 1,
        },
        {
            id: 2,
            name: "Task 3",
            crossed_by_id: 2,
            group_id: 1,
        },
    ],
};

export const fetchTasks = createAsyncThunk("/tasks", ({ token }) => {
    console.log("FETCHING TASKS");
    const resolve = fetch(`http://127.0.0.1:8080/tasks`, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${token}`,
        },
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
                return `successfully fetched tasks`;
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

export const tasksSlice = createSlice({
    name: "tasks",
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder.addCase(fetchTasks.pending, (state) => {
            state.loading = true;
        });
        builder.addCase(fetchTasks.fulfilled, (state, action) => {
            state.loading = false;
            state.data = action.payload;
        });
        builder.addCase(fetchTasks.rejected, (state, action) => {
            state.loading = false;
            console.error(action.error);
            toast.error(action.error.message);
        });
    },
});

export const {} = tasksSlice.actions;

export default tasksSlice.reducer;
