import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

const initialState = {
    loading: false,
    error: false,
    data: [],
};

export const fetchGroups = createAsyncThunk("/groups", ({ token }) => {
    console.log("FETCVHING GROUPS");
    const resolve = fetch(`http://127.0.0.1:8080/groups`, {
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
                return `successfully fetched groups`;
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

export const createGroupFtch = createAsyncThunk("/group-add", ({ token, userId, groupName }) => {
    console.log("creating new group: ", groupName);
    const resolve = fetch(`http://127.0.0.1:8080/group-add`, {
        method: "POST",
        headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ name: groupName, creator: userId }),
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
                return `successfully created new group`;
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

export const groupsSlice = createSlice({
    name: "groups",
    initialState,
    reducers: {
        forceInvite: (state, action) => {
            const newGroup = action.payload.group;
            toast.info(`you were invited to ${newGroup.name}`);
            state.data = [...state.data, newGroup];
        },
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
            state.loading = false;
            state.error = true;
            console.error(action.error);
            console.error(action.error.message);
        });

        builder.addCase(createGroupFtch.fulfilled, (state, action) => {
            state.loading = false;
            state.data = [...state.data, action.payload];
        });
        builder.addCase(createGroupFtch.rejected, (state, action) => {
            state.loading = false;
            state.error = true;
            console.error(action.error.message);
        });
    },
});

export const { forceInvite } = groupsSlice.actions;

export default groupsSlice.reducer;
