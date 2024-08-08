import * as THREE from "three";

(async () => {
    try {
        const engine = await import("./physics_engine").then((module) => module?.default);
        // 创建场景
        const scene = new THREE.Scene();

        // 创建摄像机
        const camera = new THREE.PerspectiveCamera(
            75,
            window.innerWidth / window.innerHeight,
            0.1,
            1000,
        );
        camera.position.z = 5;

        // 创建渲染器
        const renderer = new THREE.WebGLRenderer();
        renderer.setSize(window.innerWidth, window.innerHeight);
        document.body.appendChild(renderer.domElement);

        // 创建立方体
        const geometry = new THREE.BoxGeometry();
        const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
        let static_body = engine.RigidBody.new_box(
            engine.Boxes.new(0.5, 0.5, 0.5),
            1.0,
        )!.with_type(engine.RigidBodyType.Static);
        let dynamic_body = engine.RigidBody.new_box(
            engine.Boxes.new(0.5, 0.5, 0.5),
            1.0,
        )!;

        const cube = new THREE.Mesh(geometry, material);
        scene.add(cube);
        mesh_set_position(cube, dynamic_body.position);

        let distance_joint = engine.DistanceJointThreejs.new(-1, cube.id)
            .with_length(
                2.0,
            ).with_local_pos2(engine.Vec3.new(0.5, 0.5, 0.5));
        const clock = new THREE.Clock();
        function animate() {
            requestAnimationFrame(animate);
            const delta = clock.getDelta();
            distance_joint.apply(static_body, dynamic_body);
            let gravity = engine.Vec3.new(0.0, -9.8 * dynamic_body.mass, 0.0);
            dynamic_body.apply_force(gravity, dynamic_body.position);
            dynamic_body.update(delta);

            mesh_set_position(cube, dynamic_body.position);
            mesh_set_quaternion(cube, dynamic_body.quaternion);
            const point = dynamic_body.local_position_2_world(
                engine.Vec3.new(0.5, 0.5, 0.5),
            );
            const line = new THREE.Line(
                new THREE.BufferGeometry().setFromPoints([
                    new THREE.Vector3(0.0, 0.0, 0.0),
                    new THREE.Vector3(point.x, point.y, point.z),
                ]),
                material,
            );
            scene.add(line);
            renderer.render(scene, camera);
            scene.remove(line);
        }

        animate();

        // 处理窗口尺寸变化
        window.addEventListener("resize", () => {
            camera.aspect = window.innerWidth / window.innerHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(window.innerWidth, window.innerHeight);
        });

        function mesh_set_position(mesh: THREE.Mesh, pos: any) {
            mesh.position.x = pos.x;
            mesh.position.y = pos.y;
            mesh.position.z = pos.z;
        }

        function mesh_set_quaternion(
            mesh: THREE.Mesh,
            quaternion: any,
        ) {
            mesh.quaternion.x = quaternion.x;
            mesh.quaternion.y = quaternion.y;
            mesh.quaternion.z = quaternion.z;
            mesh.quaternion.w = quaternion.w;
        }
    } catch (error) {
        console.error("Error loading WASM module:", error);
    }
})();
