import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";

const initialState = {
    loading: false,
    data: []
};

export const fetchGroups = createAsyncThunk(
    '/groups',
    ({token}) => {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                const test = Math.floor(Math.random() * 10);
                if (test < 5) {
                    resolve([{
                        id: 0,
                        title: "Group1",
                        owner: 1
                    },
                    {
                        id: 1,
                        title: "Group2",
                        owner: 1
                    }]);
                } else {
                    reject('Failed to fetch')
                }
            },2000)
        })
    }
);

export const groupsSlice = createSlice({
    name: "groups",
    initialState,
    reducers: {

    },
    extraReducers: (builder) => {
        builder.addCase(fetchGroups.pending, (state) => {
            state.loading = true;
        });
        builder.addCase(fetchGroups.fulfilled, (state, action) => {
            state.loading = false;
            state.data = action.payload;
        });
        builder.addCase(fetchGroups.rejected, (state, action) => {
            console.error(action.error.message);
        });

    },
});

export const {} = groupsSlice.actions;

export default groupsSlice.reducer;