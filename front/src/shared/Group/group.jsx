import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { TasksList } from "../TasksList/taskList";
import { toast } from "react-toastify";
import { Wraper } from "../../shared/Wrapper/Wrapper";
import "react-toastify/dist/ReactToastify.css";
import { createTaskFtch } from "../../redux/reducers/tasks";
import { InviteUsers } from "./inviteUsers";

export const Group = ({ groupid, name, creator }) => {
    const [creatorName, setCreatorName] = useState("");
    const user = useSelector((state) => state.user);
    const [visibility, setVisibility] = useState(false);
    const [visibility2, setVisibility2] = useState(false);
    const [createTaskName, setCreateTaskName] = useState("");
    const dispatch = useDispatch();
    useEffect(() => {
        console.log("getting there", groupid, name, creator);
        if (creator)
            fetch(`http://127.0.0.1:8080/user/${creator}`, {
                method: "GET",
                headers: {
                    Authorization: `Bearer ${user.jwt}`,
                },
            })
                .then(async (data) => {
                    if (!data.ok) {
                        console.log(data);
                        throw new Error(`${data.status}: ${await data.text()}`);
                    }
                    console.log("dsaf");
                    const userData = await data.json();
                    setCreatorName(userData.name);
                })
                .catch((err) => {
                    console.error(err);
                    toast.error(err.message);
                });
    }, []);

    const handleChange = (e) => {
        let { value } = e.target;
        setCreateTaskName(value);
    };

    const hideWrapper = (e) => {
        e.preventDefault();
        setVisibility(false);
        setCreateTaskName("");
    };

    const createTask = (e) => {
        e.preventDefault();
        if (createTaskName.length <= 0) return toast.error("Group name can't be empty");

        dispatch(createTaskFtch({ token: user.jwt, name: createTaskName, groupId: groupid }));

        setVisibility(false);
        setCreateTaskName("");
    };
    return (
        <>
            <div className="group">
                <button onClick={() => setVisibility(true)}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="10"
                        height="10"
                        viewBox="0 0 24 24"
                    >
                        <path d="M24 10h-10v-10h-4v10h-10v4h10v10h4v-10h10z" />
                    </svg>
                </button>
                <button onClick={() => setVisibility2(true)}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                    >
                        <path d="M19.5 15c-2.483 0-4.5 2.015-4.5 4.5s2.017 4.5 4.5 4.5 4.5-2.015 4.5-4.5-2.017-4.5-4.5-4.5zm2.5 5h-2v2h-1v-2h-2v-1h2v-2h1v2h2v1zm-7.18 4h-14.815l-.005-1.241c0-2.52.199-3.975 3.178-4.663 3.365-.777 6.688-1.473 5.09-4.418-4.733-8.729-1.35-13.678 3.732-13.678 6.751 0 7.506 7.595 3.64 13.679-1.292 2.031-2.64 3.63-2.64 5.821 0 1.747.696 3.331 1.82 4.5z" />
                    </svg>
                </button>
                <h2> {name} </h2>
                <p>Created by: {creatorName}</p>
                <TasksList groupId={groupid} />
            </div>
            {visibility && (
                <Wraper visibilityChange={setVisibility}>
                    <form action="">
                        <h1>Create new Task for group "{name}"</h1>
                        <input
                            name=""
                            value={createTaskName}
                            onChange={(e) => handleChange(e)}
                            type="text"
                            placeholder="Group Name"
                        />
                        <div className="button-space">
                            <button onClick={(e) => createTask(e)}>Create</button>
                            <button onClick={(e) => hideWrapper(e)}>Cancel</button>
                        </div>
                    </form>
                </Wraper>
            )}
            {visibility2 && (
                <Wraper visibilityChange={setVisibility2}>
                    <InviteUsers
                        groupId={groupid}
                        groupName={name}
                        setVisibility={() => setVisibility2(false)}
                    />
                </Wraper>
            )}
        </>
    );
};
