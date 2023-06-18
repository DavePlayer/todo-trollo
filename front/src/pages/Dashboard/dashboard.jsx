import { useDispatch, useSelector } from "react-redux";
import { Navbar } from "../../shared/Navbar/navbar";
import { Group } from "../../shared/Group/group";
import { useEffect } from "react";
import { fetchGroups } from "../../redux/reducers/groups";

export const Dashboard = () => {
    const groups = useSelector((state) => state.groups);
    const user = useSelector((state) => state.user);
    const dispatch = useDispatch();
    useEffect(() => {
        dispatch(fetchGroups({ token: user.jwt }));
    },[])
    return(
        <>
            {groups.loading ? (
                <div className="dashboard">
                    <Navbar/>
                    <div className="mainspace">
                        <p>Loading...</p>
                    </div>
                </div>
            ): (
                <div className="dashboard">
                    <Navbar/>
                    <div className="mainspace">
                        {groups.data.map(group => (<Group key={group.id} groupid={group.id} title={group.title} owner={group.owner}></Group>))}
                    </div>
                </div>
            )}
        </>
        
    );
}
