(function() {
    const canvas = document.getElementById('hero-graphs');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    let w, h;

    function resize() {
        const r = canvas.parentElement.getBoundingClientRect();
        w = canvas.width = r.width;
        h = canvas.height = r.height;
    }

    const MESH_COUNT = 10;
    const CONNECT_DIST = 90;
    const meshes = [];

    function makeMesh() {
        // Start near an edge but inside
        const edge = Math.floor(Math.random() * 4);
        let cx, cy;
        const margin = 80;
        if (edge === 0) {        // left
            cx = Math.random() * margin;
            cy = Math.random() * h;
        } else if (edge === 1) {  // right
            cx = w - Math.random() * margin;
            cy = Math.random() * h;
        } else if (edge === 2) {  // top
            cx = Math.random() * w;
            cy = Math.random() * margin;
        } else {                  // bottom
            cx = Math.random() * w;
            cy = h - Math.random() * margin;
        }
        const nodeCount = 4 + Math.floor(Math.random() * 5);
        const nodes = [];
        for (let i = 0; i < nodeCount; i++) {
            nodes.push({
                ox: (Math.random() - 0.5) * 120,
                oy: (Math.random() - 0.5) * 100,
                r: 1 + Math.random() * 1.5,
                wobblePhase: Math.random() * Math.PI * 2,
                wobbleAmp: 3 + Math.random() * 8,
                wobbleSpeed: 0.3 + Math.random() * 0.4,
            });
        }
        return {
            cx, cy,
            dx: (Math.random() - 0.5) * 0.15,
            dy: (Math.random() - 0.5) * 0.15,
            rotation: Math.random() * Math.PI * 2,
            rotSpeed: (Math.random() - 0.5) * 0.0002,
            alpha: 0.08 + Math.random() * 0.09,
            nodes,
        };
    }

    function init() {
        resize();
        meshes.length = 0;
        const MIN_DIST = 200;
        for (let i = 0; i < MESH_COUNT; i++) {
            for (let attempt = 0; attempt < 50; attempt++) {
                const m = makeMesh();
                const tooClose = meshes.some(other => {
                    const dx = m.cx - other.cx;
                    const dy = m.cy - other.cy;
                    return Math.sqrt(dx * dx + dy * dy) < MIN_DIST;
                });
                if (!tooClose || attempt === 49) {
                    meshes.push(m);
                    break;
                }
            }
        }
    }

    function draw(t) {
        ctx.clearRect(0, 0, w, h);
        const theme = document.documentElement.getAttribute('data-theme') || '';
        const isDark = theme.includes('business') || theme.includes('dark');
        const dot  = isDark ? '150,180,230' : '30,50,120';
        const line = isDark ? '130,160,210' : '40,60,110';
        const ts = t * 0.001;

        for (const m of meshes) {
            m.cx += m.dx;
            m.cy += m.dy;
            m.rotation += m.rotSpeed;
            if (m.cx < -150) m.cx = w + 150;
            if (m.cx > w + 150) m.cx = -150;
            if (m.cy < -150) m.cy = h + 150;
            if (m.cy > h + 150) m.cy = -150;

            const cos = Math.cos(m.rotation);
            const sin = Math.sin(m.rotation);

            const pts = m.nodes.map(n => {
                const wobX = Math.sin(ts * n.wobbleSpeed + n.wobblePhase) * n.wobbleAmp;
                const wobY = Math.cos(ts * n.wobbleSpeed * 0.8 + n.wobblePhase + 1) * n.wobbleAmp;
                const lx = n.ox + wobX;
                const ly = n.oy + wobY;
                return {
                    x: m.cx + lx * cos - ly * sin,
                    y: m.cy + lx * sin + ly * cos,
                    r: n.r,
                };
            });

            for (let i = 0; i < pts.length; i++) {
                for (let j = i + 1; j < pts.length; j++) {
                    const dx = pts[i].x - pts[j].x;
                    const dy = pts[i].y - pts[j].y;
                    const dist = Math.sqrt(dx * dx + dy * dy);
                    if (dist < CONNECT_DIST) {
                        const fade = 1 - dist / CONNECT_DIST;
                        ctx.beginPath();
                        ctx.moveTo(pts[i].x, pts[i].y);
                        ctx.lineTo(pts[j].x, pts[j].y);
                        ctx.strokeStyle = `rgba(${line},${m.alpha * fade})`;
                        ctx.lineWidth = 0.6;
                        ctx.stroke();
                    }
                }
            }

            for (const p of pts) {
                ctx.beginPath();
                ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
                ctx.fillStyle = `rgba(${dot},${m.alpha * 1.5})`;
                ctx.shadowBlur = p.r * 4;
                ctx.shadowColor = `rgba(${dot},${m.alpha})`;
                ctx.fill();
            }
        }
        ctx.shadowBlur = 0;
        requestAnimationFrame(draw);
    }

    window.addEventListener('resize', init);
    init();
    requestAnimationFrame(draw);
})();
