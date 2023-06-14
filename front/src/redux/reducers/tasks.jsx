import { createSlice } from "@reduxjs/toolkit";

const initialState = {
    loading: false,
    data: [
        {
            id: 0,
            title: "Task1",
            text: "Lorem Ipsum",
            groupid: 1,
            owner: 1,
            isCrossed: false
        },
        {
            id: 1,
            title: "Task2",
            text: "Lorem Ipsum",
            groupid: 1,
            owner: 1,
            isCrossed: false
        },
        {
            id: 2,
            title: "Task3",
            text: "Lorem Ipsum",
            groupid: 1,
            owner: 1,
            isCrossed: true
        }
    ]
}

export const tasksSlice = createSlice({
    name: "tasks",
    initialState,
    reducers: {

    },
})

export const {} = tasksSlice.actions;

export default tasksSlice.reducer;