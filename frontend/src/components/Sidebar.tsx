import { Accessor, Component, createSignal, JSX, Show } from "solid-js";
import { Link } from "@tanstack/solid-router";
import { clsx } from "clsx";

import { Icon } from "./ui/Icon";

type SidebarItemProps = {
  href: string;
  label: string;
  iconName: string;
  isExpanded: Accessor<boolean>;
};

const SidebarListItem: Component<SidebarItemProps> = (props) => {
  const linkClass = () => {
    return clsx("h-12 group flex items-center hover:bg-gray-400/10 rounded-md", {
      "w-12 justify-center": !props.isExpanded(),
      "w-full gap-x-2 px-2": props.isExpanded(),
    });
  };

  return (
    <li class="contents">
      <Link to={props.href} class={linkClass()}>
        <Icon name={props.iconName} class="group-hover:text-blue-700" />
        <Show when={props.isExpanded()}>
          <span class="text-sm font-medium capitalize group-hover:text-blue-700">
            {props.label}
          </span>
        </Show>
      </Link>
    </li>
  );
};

type SidebarListProps = {
  label: string;
  isExpanded: Accessor<boolean>;
  children: JSX.Element;
};

const SidebarList: Component<SidebarListProps> = (props) => (
  <div class="relative pt-6">
    <Show when={props.isExpanded()}>
      <div class="absolute top-0 translate-y-1/2 px-6 text-xs capitalize">{props.label}</div>
    </Show>
    <ul
      class={clsx("flex flex-col items-center gap-y-2", {
        "px-4": props.isExpanded(),
      })}
    >
      {props.children}
    </ul>
  </div>
);

const SidebarListDivider: Component = () => <div class="h-[1px] w-full bg-gray-100"></div>;

export const Sidebar: Component = () => {
  const [isExpanded, setIsExpanded] = createSignal(false);

  return (
    <aside
      class={clsx("flex flex-col gap-y-2 border-r border-r-gray-200 bg-white py-2", {
        "w-16": !isExpanded(),
        "w-56": isExpanded(),
      })}
      on:dblclick={() => setIsExpanded((prev) => !prev)}
    >
      <SidebarList label="vulnerabilities" isExpanded={isExpanded}>
        <SidebarListItem
          href="/vulnerabilities"
          label="analytics"
          iconName="bar_chart"
          isExpanded={isExpanded}
        />
        <SidebarListItem
          href="/vulnerabilities/search"
          label="search"
          iconName="search"
          isExpanded={isExpanded}
        />
        <SidebarListItem
          href="/vulnerabilities/alerts"
          label="alerts"
          iconName="alarm"
          isExpanded={isExpanded}
        />
      </SidebarList>
      <SidebarListDivider />
      <SidebarList label="account" isExpanded={isExpanded}>
        <SidebarListItem href="/" label="theme" iconName="light_mode" isExpanded={isExpanded} />
        <SidebarListItem
          href="/settings"
          label="settings"
          iconName="settings"
          isExpanded={isExpanded}
        />

        <SidebarListItem href="/" label="log out" iconName="logout" isExpanded={isExpanded} />
      </SidebarList>
    </aside>
  );
};
