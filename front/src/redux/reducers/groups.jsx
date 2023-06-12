import { createSlice } from "@reduxjs/toolkit";

const initialState = {
    loading: true,
    data: [
        {
            id: 0,
            title: "Group1",
            owner: 1
        },
        {
            id: 1,
            title: "Group2",
            owner: 1
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