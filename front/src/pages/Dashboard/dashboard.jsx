import { useSelector } from "react-redux";
import { Navbar } from "../../shared/Navbar/navbar";
import { Group } from "../../shared/Group/group";

export const Dashboard = () => {
    const groups = useSelector((state) => state.groups);
    return(
        <div className="dashboard">
            <Navbar/>
            <div className="mainspace">
                {groups.loading && <p>Loading...</p>}
                {!groups.loading && groups.data.map(group => (
                    <Group key={group.id} groupid={group.id} title={group.title} owner={group.owner}></Group>
                ))}
            </div>
        </div>
    );
}
