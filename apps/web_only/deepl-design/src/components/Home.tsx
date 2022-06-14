import {
    Alert,
    AlertProps,
    Button,
    Container,
    Dialog,
    DialogProps,
    Grid,
    IconButton,
    Paper,
    Stack,
    styled,
    Tabs,
    Tab,
} from "@mui/material";
import * as mobileNet from "@tensorflow-models/mobilenet";
import { MobileNet } from "@tensorflow-models/mobilenet";
import {
    MutableRefObject,
    ReactNode,
    useEffect,
    useRef,
    useState,
} from "react";
import { useVideo } from "react-use";
import { List } from "immutable";
import AddAPhotoIcon from "@mui/icons-material/AddAPhoto";
import "@tensorflow/tfjs-backend-cpu";
import "@tensorflow/tfjs-backend-webgl";

const Item = styled(Alert)(({ theme }) => ({
    ...theme.typography.body2,
    padding: theme.spacing(1),
    textAlign: "center",
    width: "100%",
    overflow: "auto",
}));

const Root = styled(Container)(() => ({
    minHeight: "100vh",
}));

enum LoadStatus {
    LoadVideoElement,
    LoadModel,
    LoadCanvas,
    CaptureImage,
    Classifing,
    Done,
}

function AlertItem({
    node,
    children,
    ...rest
}: { node: React.Key; children?: ReactNode } & AlertProps): JSX.Element {
    const value = children === undefined ? node : children;
    return (
        <Paper className="w-full">
            <Item {...rest}>{value}</Item>
        </Paper>
    );
}

AlertItem.protoTypes = {};

function CanvasDialog(
    props: {
        canvasRef: MutableRefObject<HTMLCanvasElement | null>;
        videoRef: MutableRefObject<HTMLVideoElement | null>;
        modelRef: MutableRefObject<MobileNet | null | undefined>;
        setResult: (
            datas: { className: string; probability: number }[]
        ) => void;
    } & DialogProps
): JSX.Element {
    const { videoRef, canvasRef, modelRef, setResult, ...rest } = props;
    useEffect(() => {
        if (videoRef.current !== null) {
            videoRef.current.oncanplay = () => {
                if (canvasRef.current !== null && videoRef.current !== null) {
                    canvasRef.current.width = videoRef.current.videoWidth;
                    canvasRef.current.height = videoRef.current.videoHeight;
                }
            };
        }
    }, []);
    const classify = async () => {
        if (
            modelRef.current !== null &&
            modelRef.current !== undefined &&
            canvasRef.current !== null
        ) {
            const ctx = canvasRef.current.getContext("2d");
            if (ctx !== null) {
                const image = ctx.getImageData(
                    0,
                    0,
                    canvasRef.current.width,
                    canvasRef.current.height
                );
                const result = await modelRef.current.classify(image);
                setResult(result);
            }
        }
    };
    return (
        <Dialog {...rest}>
            <Button onClick={() => classify()}>classify</Button>
            <canvas ref={canvasRef} />
        </Dialog>
    );
}

function Home() {
    const [video, _state, _controls, ref] = useVideo(<video autoPlay />);
    const model = useRef<MobileNet | null | undefined>();
    const [_, setLoading] = useState(LoadStatus.LoadVideoElement);
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const [videoDevices, setVideoDevices] = useState<Array<MediaDeviceInfo>>();
    const [videoDevice, setVideoDevice] = useState(0);
    const [open, setOpen] = useState(false);
    const [Alerts, setAlerts] = useState(
        List([
            <AlertItem
                severity="info"
                key="init video media"
                node="init video media"
            />,
        ])
    );
    useEffect(() => {
        const f = async () => {
            const devices = await navigator.mediaDevices.enumerateDevices();
            const videoDevices = devices.filter((v) => v.kind === "videoinput");
            setVideoDevices(videoDevices);
            if (videoDevices.length !== 0) {
                setVideoDevice(0);
            }
        };
        f();
        if (model.current === undefined) {
            model.current = null;
            mobileNet
                .load()
                .then((v) => {
                    model.current = v;
                    setAlerts((stack) =>
                        stack.push(
                            <AlertItem
                                severity="success"
                                key={`${stack.size} load model successed`}
                                node={`load model successed`}
                            />
                        )
                    );
                })
                .catch((e) =>
                    setAlerts((stack) =>
                        stack.push(
                            <AlertItem
                                severity="error"
                                key={`${
                                    stack.size
                                } load model failed: ${e.toString()},retrying...`}
                                node={`load model failed: ${e.toString()},retrying...`}
                            />
                        )
                    )
                );
        }
    }, []);
    useEffect(() => {
        if (videoDevices !== undefined) {
            if (ref.current !== null) {
                navigator.mediaDevices
                    .getUserMedia({
                        audio: false,
                        video: {
                            deviceId: videoDevices[videoDevice].deviceId,
                        },
                    })
                    .then((v) => {
                        if (
                            ref.current !== null
                        ) {
                            ref.current.srcObject = v;
                            setLoading(LoadStatus.LoadCanvas);
                            setAlerts((stack) =>
                                stack.push(
                                    <AlertItem
                                        severity="success"
                                        key={`${stack.size} load video successed`}
                                        node={`load video successed`}
                                    />
                                )
                            );
                        }
                    })
                    .catch((e) =>
                        setAlerts((stack) =>
                            stack.push(
                                <AlertItem
                                    severity="error"
                                    key={`${
                                        stack.size
                                    } load video failed: ${e.toString()}`}
                                    node={`load video failed: ${e.toString()}`}
                                />
                            )
                        )
                    );
            }
        }
    }, [videoDevices, videoDevice]);
    const capture = async () => {
        if (canvasRef.current !== null && ref.current !== null) {
            const ctx = canvasRef.current.getContext("2d");
            if (ctx !== null) {
                ctx.drawImage(ref.current, 0, 0);
            } else {
                setAlerts((stack) =>
                    stack.push(
                        <AlertItem
                            severity="error"
                            key={`${stack.size} can not get canvas context!`}
                            node="can not get canvas context!"
                        />
                    )
                );
            }
        } else {
            setAlerts((stack) =>
                stack.push(
                    <AlertItem
                        severity="error"
                        key={`${stack.size} ${
                            canvasRef.current === null ? "canvas" : "video"
                        } not init yet!`}
                        node={`${
                            canvasRef.current === null ? "canvas" : "video"
                        } not init yet!`}
                    />
                )
            );
        }
    };
    return (
        <Root>
            <CanvasDialog
                canvasRef={canvasRef}
                videoRef={ref}
                modelRef={model}
                open={true}
                className={`${open ? "" : "hidden"}`}
                onClose={() => setOpen(false)}
                setResult={(datas) =>
                    setAlerts((stack) =>
                        stack.push(
                            ...datas.map(({ className, probability }) => (
                                <AlertItem
                                    key={`${className}${probability}`}
                                    node={`about ${(probability * 100).toFixed(
                                        2
                                    )}% is ${className}`}
                                />
                            ))
                        )
                    )
                }
            />
            <Grid container columns={{ xs: 4, md: 8, sm: 8 }} spacing={2}>
                <Grid item className="w-full h-auto">
                    <Tabs
                        value={videoDevice}
                        onChange={(_, v) => {
                            setVideoDevice(v);
                        }}
                    >
                        {videoDevices?.map((v) => <Tab key={v.deviceId} label={v.label} />) ??
                            []}
                    </Tabs>
                </Grid>
                <Grid
                    className="group relative cursor-pointer"
                    item
                    xs={4}
                >
                    <div className="hidden absolute w-full h-full group-hover:flex justify-center items-center">
                        <IconButton
                            className="w-32 h-32 z-10"
                            onClick={() => {
                                setOpen(true);
                                capture();
                            }}
                        >
                            <AddAPhotoIcon sx={{ fontSize: 32 }} />
                        </IconButton>
                    </div>
                    {video}
                </Grid>
                <Grid style={{height: '50vh'}} className="w-full overflow-scroll" item xs={4}>
                    <Stack spacing={2} className="w-full h-auto">
                        {Alerts.toArray()}
                    </Stack>
                </Grid>
            </Grid>
        </Root>
    );
}

Home.propTypes = {};

export default Home;
