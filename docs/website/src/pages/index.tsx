import type { ReactNode } from "react";
import clsx from "clsx";
import Link from "@docusaurus/Link";
import Translate, { translate } from "@docusaurus/Translate";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import Heading from "@theme/Heading";

import styles from "./index.module.css";

function AlphaBanner(): ReactNode {
  return (
    <div className={styles.alphaBanner}>
      <Translate id="homepage.alphaBanner">
        This documentation is for Desktop Homunculus v0.1.0-alpha. APIs may
        change.
      </Translate>
    </div>
  );
}

function HeroSection(): ReactNode {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx("hero hero--primary", styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className={styles.heroTitle}>
          {siteConfig.title}
        </Heading>
        <p className={styles.heroSubtitle}>{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/getting-started"
          >
            <Translate id="homepage.hero.getStarted">Get Started</Translate>
          </Link>
        </div>
      </div>
    </header>
  );
}

function PersonaCards(): ReactNode {
  const personas = [
    {
      title: translate({ id: "homepage.card.getStarted.title", message: "Get Started" }),
      to: "/getting-started",
      description: translate({ id: "homepage.card.getStarted.description", message: "Install and start using Desktop Homunculus" }),
    },
    {
      title: translate({ id: "homepage.card.buildMod.title", message: "Build a MOD" }),
      to: "/mod-development",
      description: translate({ id: "homepage.card.buildMod.description", message: "Create custom characters, UIs, and integrations" }),
    },
    {
      title: translate({ id: "homepage.card.contribute.title", message: "Contribute" }),
      to: "/contributing",
      description: translate({ id: "homepage.card.contribute.description", message: "Help improve Desktop Homunculus" }),
    },
  ];

  return (
    <section className="container">
      <div className={styles.cards}>
        {personas.map((persona) => (
          <Link key={persona.title} to={persona.to} className={styles.card}>
            <div className={styles.cardTitle}>{persona.title}</div>
            <div className={styles.cardDescription}>
              {persona.description}
            </div>
          </Link>
        ))}
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={siteConfig.title}
      description={translate({
        id: "homepage.description",
        message: "Documentation for Desktop Homunculus — your AI-powered desktop companion",
      })}
    >
      <AlphaBanner />
      <HeroSection />
      <main>
        <PersonaCards />
      </main>
    </Layout>
  );
}
