import { motion } from "framer-motion";

interface WaveformProps {
  active: boolean;
  bars?: number;
}

export default function Waveform({ active, bars = 5 }: WaveformProps) {
  return (
    <div className="flex items-end gap-[2px] h-4">
      {Array.from({ length: bars }).map((_, i) => (
        <motion.div
          key={i}
          className="w-[2px] rounded-full bg-red-400"
          animate={
            active
              ? {
                  height: [4, 16, 6, 14, 8, 16, 4][i % 7] ?? 8,
                  opacity: [0.5, 1, 0.4, 0.9, 0.6, 1, 0.5][i % 7] ?? 0.6,
                }
              : { height: 4, opacity: 0.3 }
          }
          transition={
            active
              ? {
                  duration: 0.6 + i * 0.08,
                  repeat: Infinity,
                  ease: "easeInOut",
                  delay: i * 0.1,
                }
              : { duration: 0.2 }
          }
        />
      ))}
    </div>
  );
}
