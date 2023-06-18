import { useDispatch, useSelector } from "react-redux";
import { Navbar } from "../../shared/Navbar/navbar";
import { Group } from "../../shared/Group/group";
import { useEffect } from "react";
import { fetchGroups } from "../../redux/reducers/groups";
import { fetchTasks } from "../../redux/reducers/tasks";

export const Dashboard = () => {
    const groups = useSelector((state) => state.groups);
    const user = useSelector((state) => state.user);
    const dispatch = useDispatch();
    useEffect(() => {
        dispatch(fetchGroups({ token: user.jwt }));
        dispatch(fetchTasks({ token: user.jwt }));
    }, []);
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
                                <button>Create new Group</button>
                            </header>
                            {groups.data.map((group) => (
                                <Group
                                    key={group.id}
                                    groupid={group.id}
                                    name={group.name}
                                    creator={group.creator}
                                ></Group>
                            ))}
                        </div>
                    </div>
                </section>
            )}
        </>
    );
};
