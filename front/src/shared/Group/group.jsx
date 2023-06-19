import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { TasksList } from "../TasksList/taskList";
import { toast } from "react-toastify";
import { Wraper } from "../../shared/Wrapper/Wrapper";
import "react-toastify/dist/ReactToastify.css";
import { createTaskFtch } from "../../redux/reducers/tasks";

export const Group = ({ groupid, name, creator }) => {
    const [creatorName, setCreatorName] = useState("");
    const user = useSelector((state) => state.user);
    const [visibility, setVisibility] = useState(false);
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
        </>
    );
};
