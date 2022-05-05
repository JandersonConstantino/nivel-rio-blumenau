#!/usr/bin/env node
"use strict";

const axios = require("axios");
const meow = require("meow");
const Spinner = require("cli-spinner").Spinner;
const fns = require("date-fns");

const CLI_TEMPLATE = `
Utilização
    $ nivel-rio-blumenau
Opções
    --last, -l    Última medição
Exemplos
    $ nivel-rio-blumenau -l
        01/12/2022 18:00 - 1.29 metros (-0.08)
    $ nivel-rio-blumenau
        01/12/2022 13:00 - 1.29 metros (-0.08)
        01/12/2022 14:00 - 1.28 metros (-0.01)
        ..
        01/12/2022 21:00 - 1.29 metros (+0.01)
        01/12/2022 22:00 - 1.40 metros (+0.11)
`;

async function main() {
  const cli = meow(CLI_TEMPLATE, { alias: { l: "last" } });

  const spinner = new Spinner("Fetching data...");
  spinner.setSpinnerString(0);
  spinner.start();

  const onlyLastItem = !!cli.flags.last;

  const { data } = await axios.get(
    "https://alertablu.blumenau.sc.gov.br/static/data/nivel_oficial.json"
  );

  const items = ((data && data.niveis) || []).map((item) => ({
    nivel: item.nivel,
    horaLeitura: fns.format(
      new Date(item.horaLeitura.replace("Z", "")),
      "dd/MM/yyyy HH:MM"
    ),
  }));

  if (onlyLastItem && items.length > 1) {
    items.length = 1;
  }

  spinner.stop(true);
  console.table(items);
}

main();
