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

export const groupsSlice = createSlice({
    name: "groups",
    initialState,
    reducers: {

    },
})

export const {} = groupsSlice.actions;

export default groupsSlice.reducer;