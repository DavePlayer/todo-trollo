import { useDispatch, useSelector } from "react-redux";
import { Navbar } from "../../shared/Navbar/navbar";
import { Group } from "../../shared/Group/group";
import { useEffect, useState, useCallback } from "react";
import { createGroupFtch, fetchGroups, forceInvite } from "../../redux/reducers/groups";
import { createTaskWS, fetchTasks, updateTaskWS } from "../../redux/reducers/tasks";
import { Wraper } from "../../shared/Wrapper/Wrapper";
import { toast } from "react-toastify";
import useWebSocket, { ReadyState } from "react-use-websocket";

function isJsonString(str) {
    try {
        JSON.parse(str);
    } catch (e) {
        return false;
    }
    return true;
}

export const Dashboard = () => {
    const groups = useSelector((state) => state.groups);
    const user = useSelector((state) => state.user);
    const tasks = useSelector((state) => state.tasks);
    const [visibility, setVisibility] = useState(false);
    const [createGroupName, setCreateGroupName] = useState("");
    const dispatch = useDispatch();
    const [messageHistory, setMessageHistory] = useState([]);

    const { sendMessage, lastMessage, readyState } = useWebSocket("ws://127.0.0.1:8080/ws/");
    const connectionStatus = {
        [ReadyState.CONNECTING]: "Connecting",
        [ReadyState.OPEN]: "Open",
        [ReadyState.CLOSING]: "Closing",
        [ReadyState.CLOSED]: "Closed",
        [ReadyState.UNINSTANTIATED]: "Uninstantiated",
    }[readyState];

    useEffect(() => {
        if (lastMessage !== null) {
            setMessageHistory((prev) => prev.concat(lastMessage));
            var i = lastMessage.data.indexOf(" ");
            let [command, commandData] = [
                lastMessage.data.slice(0, i),
                lastMessage.data.slice(i + 1),
            ];

            if (isJsonString(commandData)) {
                commandData = JSON.parse(commandData);

                switch (command) {
                    case "/taskCrossed":
                    case "/taskUncrossed":
                        dispatch(updateTaskWS(commandData));
                        break;
                    case "/forceInvite":
                        console.log("force inviting user: ", commandData);
                        if (commandData.user_id == user.id) {
                            dispatch(forceInvite(commandData));
                        }
                        break;
                    case "/taskCreate":
                        console.log("Someone created new task: ", command);
                        dispatch(createTaskWS(commandData));
                        break;
                }
            }
        }
    }, [lastMessage, setMessageHistory]);

    useEffect(() => {
        dispatch(fetchGroups({ token: user.jwt }));
        dispatch(fetchTasks({ token: user.jwt }));
        toast.info(`websocket status: ${connectionStatus}`);
    }, []);

    const handleChange = (e) => {
        let { value } = e.target;
        setCreateGroupName(value);
    };

    const hideWrapper = (e) => {
        e.preventDefault();
        setVisibility(false);
        setCreateGroupName("");
    };

    const createGroup = (e) => {
        e.preventDefault();
        if (createGroupName.length <= 0) return toast.error("Group name can't be empty");

        dispatch(createGroupFtch({ token: user.jwt, userId: user.id, groupName: createGroupName }));

        setVisibility(false);
        setCreateGroupName("");
    };

    return (
        <>
            {groups.loading ? (
                <section className="dash-wrapper">
                    <div className="dashboard">
                        <div className="mainspace">
                            <p>Loading ...</p>
                        </div>
                    </div>
                </section>
            ) : (
                <section className="dash-wrapper">
                    <div className="dashboard">
                        <div className="mainspace">
                            <header>
                                <button onClick={() => setVisibility(true)}>
                                    Create new Group
                                </button>
                            </header>
                            <section className="group-box">
                                {groups.data.map((group) => (
                                    <Group
                                        key={group.id}
                                        groupid={group.id}
                                        name={group.name}
                                        creator={group.creator}
                                    ></Group>
                                ))}
                            </section>
                        </div>
                    </div>
                </section>
            )}
            {visibility && (
                <Wraper visibilityChange={setVisibility}>
                    <form action="">
                        <h1>Create new Group</h1>
                        <input
                            name=""
                            value={createGroupName}
                            onChange={(e) => handleChange(e)}
                            type="text"
                            placeholder="Group Name"
                        />
                        <div className="button-space">
                            <button onClick={(e) => createGroup(e)}>Create</button>
                            <button onClick={(e) => hideWrapper(e)}>Cancel</button>
                        </div>
                    </form>
                </Wraper>
            )}
        </>
    );
};
