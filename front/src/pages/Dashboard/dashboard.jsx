import { useDispatch, useSelector } from "react-redux";
import { Navbar } from "../../shared/Navbar/navbar";
import { Group } from "../../shared/Group/group";
import { useEffect, useState } from "react";
import { createGroupFtch, fetchGroups } from "../../redux/reducers/groups";
import { fetchTasks } from "../../redux/reducers/tasks";
import { Wraper } from "../../shared/Wrapper/Wrapper";
import { toast } from "react-toastify";

export const Dashboard = () => {
    const groups = useSelector((state) => state.groups);
    const user = useSelector((state) => state.user);
    const tasks = useSelector((state) => state.tasks);
    const [visibility, setVisibility] = useState(false);
    const [createGroupName, setCreateGroupName] = useState("");
    const dispatch = useDispatch();
    useEffect(() => {
        dispatch(fetchGroups({ token: user.jwt }));
        dispatch(fetchTasks({ token: user.jwt }));
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
