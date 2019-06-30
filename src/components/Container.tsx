import React from "react";

export type Props = {
  title: React.ReactNode;
};

export const Container: React.FC<Props> = ({ title, children }) => (
  <div className="nes-container with-title">
    <p className="title">{title}</p>
    {children}
  </div>
);
