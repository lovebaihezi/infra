import { Container, Fab, Grid } from "@mui/material";
import KeyboardVoiceIcon from "@mui/icons-material/KeyboardVoice";
import SettingsVoiceIcon from "@mui/icons-material/SettingsVoice";
import { FC } from "react";
import { useRecoilState } from "recoil";
import { IsRecord } from "../states/record";

const MicroButton: FC = () => {
    const [isRecord, switcher] = useRecoilState(IsRecord);
    return (
        <div
            className="fixed bottom-5 right-5"
            onClick={() => switcher(!isRecord)}
        >
            <Fab color="primary" aria-label="add">
                {isRecord ? <SettingsVoiceIcon /> : <KeyboardVoiceIcon />}
            </Fab>
        </div>
    );
};

const Home: FC = () => {
    return (
        <div className="w-screen h-screen relative">
            <Container>
                <Grid container spacing={2}></Grid>
            </Container>
            <MicroButton />
        </div>
    );
};

Home.propTypes = {};

export default Home;
