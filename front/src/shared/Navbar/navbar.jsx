import { Link } from "react-router-dom";

export const Navbar = () => {
    return(
        <div className="navbar">
            <h1>Todo trollo</h1>
            <Link to={'/logout'}>Log out</Link>
        </div>
    );
};