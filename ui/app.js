const invoke = window.__TAURI__?.core?.invoke;

const STORAGE_KEY = "rust-training-progress-v1";
const SETTINGS_KEY = "rust-training-settings-v2";
const COPY_WARNING = "Opa, calma aí! Você tem que digitar o código para aprender!";
const COPY_WARNING_THRESHOLD = 10;
const DEFAULT_SETTINGS = {
  showRubric: false,
  showHistory: true,
  reviewMode: false,
  tooltips: true,
};

const state = {
  catalog: [],
  flatExercises: [],
  currentExercise: null,
  nextHint: 1,
  organization: "natural",
  searchTerm: "",
  settings: { ...DEFAULT_SETTINGS },
  progress: {
    answers: {},
    reports: {},
    history: {},
    lastExerciseId: null,
  },
  copyWarningTimer: null,
  catalogStatusTimer: null,
};

const elements = {
  homeView: document.querySelector("#home-view"),
  exerciseView: document.querySelector("#exercise-view"),
  homeButton: document.querySelector("#home-button"),
  reloadCatalogButton: document.querySelector("#reload-catalog-button"),
  catalogStatus: document.querySelector("#catalog-status"),
  settingsButton: document.querySelector("#settings-button"),
  settingsMenu: document.querySelector("#settings-menu"),
  flagRubric: document.querySelector("#flag-rubric"),
  flagHistory: document.querySelector("#flag-history"),
  flagReview: document.querySelector("#flag-review"),
  flagTooltips: document.querySelector("#flag-tooltips"),
  startButton: document.querySelector("#start-button"),
  moduleCards: document.querySelector("#module-cards"),
  homeTotal: document.querySelector("#home-total"),
  homeModules: document.querySelector("#home-modules"),
  homeCompleted: document.querySelector("#home-completed"),
  organizationSelect: document.querySelector("#organization-select"),
  searchInput: document.querySelector("#search-input"),
  exerciseList: document.querySelector("#exercise-list"),
  moduleLabel: document.querySelector("#module-label"),
  exerciseTitle: document.querySelector("#exercise-title"),
  exercisePosition: document.querySelector("#exercise-position"),
  exercisePrompt: document.querySelector("#exercise-prompt"),
  solutionButton: document.querySelector("#solution-button"),
  hintButton: document.querySelector("#hint-button"),
  hintPanel: document.querySelector("#hint-panel"),
  rubricPanel: document.querySelector("#rubric-panel"),
  answerInput: document.querySelector("#answer-input"),
  highlightLayer: document.querySelector("#highlight-layer"),
  checkButton: document.querySelector("#check-button"),
  statusBadge: document.querySelector("#status-badge"),
  resultOutput: document.querySelector("#result-output"),
  progressPercent: document.querySelector("#progress-percent"),
  progressCount: document.querySelector("#progress-count"),
  progressDetail: document.querySelector("#progress-detail"),
  prevButton: document.querySelector("#prev-button"),
  nextButton: document.querySelector("#next-button"),
  clearProgressButton: document.querySelector("#clear-progress-button"),
  saveStateLabel: document.querySelector("#save-state-label"),
  lastRunLabel: document.querySelector("#last-run-label"),
  historyPanel: document.querySelector("#history-panel"),
  solutionModal: document.querySelector("#solution-modal"),
  solutionContent: document.querySelector("#solution-content"),
  solutionTitle: document.querySelector("#solution-title"),
  closeSolutionButton: document.querySelector("#close-solution-button"),
};

boot();

async function boot() {
  if (!invoke) {
    renderFatal("Abra este app pelo Tauri.");
    return;
  }

  state.progress = loadProgress();
  state.settings = loadSettings();
  syncSettingsControls();

  try {
    state.catalog = await invoke("get_catalog");
    rebuildFlatExercises();
    renderCatalog();
    renderDashboard();
    renderHome();
    showHomeView();
  } catch (error) {
    renderFatal(String(error));
  }
}

async function reloadCatalog() {
  const currentId = state.currentExercise?.id;
  const wasExerciseOpen = !elements.exerciseView.hidden;

  elements.reloadCatalogButton.disabled = true;
  elements.reloadCatalogButton.textContent = "Recarregando";
  showCatalogStatus("Relendo arquivos...", "busy");

  try {
    state.catalog = await invoke("get_catalog");
    rebuildFlatExercises();
    renderCatalog();
    renderDashboard();
    renderHome();

    const currentStillExists = currentId && findSummary(currentId);
    if (wasExerciseOpen && currentStillExists) {
      await selectExercise(currentId);
    } else if (wasExerciseOpen) {
      state.currentExercise = null;
      showHomeView();
    }

    showCatalogStatus(`${state.flatExercises.length} exercicios carregados`, "ok");
  } catch (error) {
    showCatalogStatus(`Falha ao recarregar: ${String(error)}`, "fail");
  } finally {
    elements.reloadCatalogButton.disabled = false;
    elements.reloadCatalogButton.textContent = "Recarregar";
  }
}

function loadProgress() {
  try {
    const parsed = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "{}");
    return {
      answers: parsed.answers ?? {},
      reports: parsed.reports ?? {},
      history: parsed.history ?? {},
      lastExerciseId: parsed.lastExerciseId ?? null,
    };
  } catch {
    return { answers: {}, reports: {}, history: {}, lastExerciseId: null };
  }
}

function saveProgress() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(state.progress));
}

function loadSettings() {
  try {
    return { ...DEFAULT_SETTINGS, ...JSON.parse(localStorage.getItem(SETTINGS_KEY) ?? "{}") };
  } catch {
    return { ...DEFAULT_SETTINGS };
  }
}

function saveSettings() {
  localStorage.setItem(SETTINGS_KEY, JSON.stringify(state.settings));
}

function rebuildFlatExercises() {
  state.flatExercises = organizedModules().flatMap((module) =>
    module.exercises.map((exercise) => ({
      ...exercise,
      module_id: module.id,
      module_title: module.title,
    })),
  );

  if (state.organization === "difficulty") {
    state.flatExercises.sort(
      (a, b) => difficultyRank(a.difficulty) - difficultyRank(b.difficulty),
    );
  }
}

function organizedModules() {
  if (state.organization === "module") {
    return [...state.catalog].sort((a, b) => a.title.localeCompare(b.title));
  }

  return state.catalog;
}

function renderCatalog() {
  elements.exerciseList.replaceChildren();
  const visible = visibleExercises();

  if (state.organization === "difficulty") {
    for (const difficulty of ["Iniciante", "Intermediario", "Avancado"]) {
      const exercises = visible.filter((exercise) => exercise.difficulty === difficulty);
      if (exercises.length === 0) continue;
      elements.exerciseList.append(
        renderExerciseGroup(difficulty, "Exercicios agrupados por dificuldade.", exercises),
      );
    }
    return;
  }

  for (const module of organizedModules()) {
    const exercises = visible.filter((exercise) => exercise.module_id === module.id);
    if (exercises.length > 0) {
      elements.exerciseList.append(renderModuleGroup(module, exercises));
    }
  }
}

function renderModuleGroup(module, visibleInModule = null) {
  const exercises = visibleInModule ?? module.exercises.map((exercise) => ({
    ...exercise,
    module_id: module.id,
    module_title: module.title,
  }));
  const completed = exercises.filter((exercise) => isCompleted(exercise.id)).length;

  return renderExerciseGroup(
    module.title,
    module.description,
    exercises,
    `${completed}/${exercises.length}`,
  );
}

function renderExerciseGroup(titleText, descriptionText, exercises, countText = null) {
  const group = document.createElement("section");
  group.className = "module-group";

  const header = document.createElement("div");
  header.className = "module-header";

  const copy = document.createElement("div");
  const title = document.createElement("h3");
  title.textContent = titleText;
  const description = document.createElement("p");
  description.textContent = descriptionText;
  copy.append(title, description);

  const count = document.createElement("span");
  count.className = "module-count";
  count.textContent =
    countText ?? `${exercises.filter((exercise) => isCompleted(exercise.id)).length}/${exercises.length}`;

  header.append(copy, count);
  group.append(header);

  const items = document.createElement("div");
  items.className = "module-items";

  for (const exercise of exercises) {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "exercise-button";
    button.dataset.exerciseId = exercise.id;
    button.title = state.settings.tooltips
      ? `${exercise.module_title ?? ""} • ${exercise.difficulty}`
      : "";
    button.addEventListener("click", () => selectExercise(exercise.id));

    const status = document.createElement("span");
    status.className = "exercise-status";
    status.dataset.state = exerciseState(exercise.id);

    const text = document.createElement("span");
    text.className = "exercise-name";
    text.textContent = exercise.title;

    const difficulty = document.createElement("span");
    difficulty.className = "difficulty-chip";
    difficulty.textContent = exercise.difficulty;

    button.append(status, text, difficulty);
    items.append(button);
  }

  group.append(items);
  return group;
}

function renderHome() {
  const visible = visibleExercises();
  const completed = state.flatExercises.filter((exercise) => isCompleted(exercise.id)).length;
  elements.homeTotal.textContent = visible.length.toString();
  elements.homeModules.textContent = state.catalog.length.toString();
  elements.homeCompleted.textContent = completed.toString();
  elements.startButton.textContent = state.progress.lastExerciseId ? "Continuar" : "Comecar";
  elements.moduleCards.replaceChildren();

  for (const module of state.catalog) {
    const moduleVisible = visible.filter((exercise) => exercise.module_id === module.id);
    if (moduleVisible.length === 0) continue;
    const completedInModule = moduleVisible.filter((exercise) => isCompleted(exercise.id)).length;
    const card = document.createElement("button");
    card.type = "button";
    card.className = "module-card";
    card.addEventListener("click", () => selectExercise(module.exercises.at(0)?.id));

    const title = document.createElement("strong");
    title.textContent = module.title;
    const description = document.createElement("span");
    description.textContent = module.description;
    const progress = document.createElement("small");
    progress.textContent = `${completedInModule}/${moduleVisible.length} concluidos`;

    card.append(title, description, progress);
    elements.moduleCards.append(card);
  }
}

async function selectExercise(id) {
  if (!id) return;

  const exercise = await invoke("get_exercise", { id });
  state.currentExercise = exercise;
  state.nextHint = 1;
  state.progress.lastExerciseId = id;
  saveProgress();

  const summary = findSummary(id);
  const module = state.catalog.find((item) => item.id === exercise.module_id);
  const index = state.flatExercises.findIndex((item) => item.id === id);

  elements.moduleLabel.textContent = module?.title ?? exercise.module_id;
  elements.exerciseTitle.textContent = exercise.title;
  elements.exercisePosition.textContent = `${index + 1} de ${state.flatExercises.length}`;
  elements.exercisePrompt.textContent = exercise.prompt;
  elements.answerInput.value = state.progress.answers[id] ?? exercise.scaffold ?? "";
  elements.hintPanel.hidden = true;
  elements.hintPanel.replaceChildren();
  renderRubric(exercise);
  elements.hintButton.disabled = exercise.hint_count === 0;
  elements.prevButton.disabled = index <= 0;
  elements.nextButton.disabled = index < 0 || index >= state.flatExercises.length - 1;

  const savedReport = state.progress.reports[id];
  if (savedReport) {
    renderReport(savedReport, { persist: false });
  } else {
    setStatus("Aguardando", "idle");
    elements.resultOutput.replaceChildren(renderEmptyResult(summary));
    elements.lastRunLabel.textContent = "Aguardando primeira verificacao";
  }
  renderHistory(id);

  renderActiveExercise();
  syncHighlight();
  showExerciseView();
  elements.answerInput.focus();
}

function renderActiveExercise() {
  document.querySelectorAll(".exercise-button").forEach((button) => {
    button.classList.toggle(
      "active",
      button.dataset.exerciseId === state.currentExercise?.id,
    );
  });
}

function showExerciseView() {
  elements.homeView.hidden = true;
  elements.exerciseView.hidden = false;
}

function showHomeView() {
  elements.exerciseView.hidden = true;
  elements.homeView.hidden = false;
  renderActiveExercise();
}

elements.prevButton.addEventListener("click", () => moveExercise(-1));
elements.nextButton.addEventListener("click", () => moveExercise(1));
elements.homeButton.addEventListener("click", showHomeView);
elements.reloadCatalogButton.addEventListener("click", reloadCatalog);
elements.settingsButton.addEventListener("click", () => {
  elements.settingsMenu.hidden = !elements.settingsMenu.hidden;
  elements.settingsButton.setAttribute("aria-expanded", String(!elements.settingsMenu.hidden));
});
elements.startButton.addEventListener("click", () => selectExercise(nextSuggestedExerciseId()));
elements.organizationSelect.addEventListener("change", () => {
  state.organization = elements.organizationSelect.value;
  rebuildFlatExercises();
  renderCatalog();
  renderDashboard();
  renderHome();
  refreshExercisePosition();
});
elements.searchInput.addEventListener("input", () => {
  state.searchTerm = elements.searchInput.value.trim().toLowerCase();
  renderCatalog();
  renderDashboard();
  renderHome();
});
for (const [element, key] of [
  [elements.flagRubric, "showRubric"],
  [elements.flagHistory, "showHistory"],
  [elements.flagReview, "reviewMode"],
  [elements.flagTooltips, "tooltips"],
]) {
  element.addEventListener("change", () => {
    state.settings[key] = element.checked;
    saveSettings();
    applySettings();
  });
}

function moveExercise(offset) {
  if (!state.currentExercise) return;
  const index = state.flatExercises.findIndex((item) => item.id === state.currentExercise.id);
  const list = visibleExercises();
  const visibleIndex = list.findIndex((item) => item.id === state.currentExercise.id);
  const target = list[visibleIndex + offset] ?? state.flatExercises[index + offset];
  if (target) selectExercise(target.id);
}

function refreshExercisePosition() {
  if (!state.currentExercise) return;
  const index = state.flatExercises.findIndex((item) => item.id === state.currentExercise.id);
  elements.exercisePosition.textContent = `${index + 1} de ${state.flatExercises.length}`;
  elements.prevButton.disabled = index <= 0;
  elements.nextButton.disabled = index < 0 || index >= state.flatExercises.length - 1;
  renderActiveExercise();
}

function nextSuggestedExerciseId() {
  if (state.progress.lastExerciseId && findSummary(state.progress.lastExerciseId)) {
    return state.progress.lastExerciseId;
  }

  const list = visibleExercises();
  return list.find((exercise) => !isCompleted(exercise.id))?.id ?? list.at(0)?.id;
}

elements.solutionButton.addEventListener("click", () => {
  if (!state.currentExercise) return;
  openSolutionModal();
});

elements.closeSolutionButton.addEventListener("click", closeSolutionModal);

elements.solutionModal.addEventListener("click", (event) => {
  if (event.target === elements.solutionModal) {
    closeSolutionModal();
  }
});

document.addEventListener("keydown", (event) => {
  if (event.key === "Escape" && !elements.solutionModal.hidden) {
    closeSolutionModal();
  }
});

document.addEventListener("copy", (event) => {
  if (selectionTouchesSolutionCode()) {
    blockSolutionCopy(event);
  }
});

document.addEventListener("selectionchange", () => {
  if (elements.solutionModal.hidden) return;
  const selectionLength = solutionCodeSelectionLength();
  if (selectionLength < COPY_WARNING_THRESHOLD) return;

  window.getSelection()?.removeAllRanges();
  showCopyWarning();
});

elements.clearProgressButton.addEventListener("click", async () => {
  const shouldClear = confirm("Limpar respostas, resultados e progresso salvo?");
  if (!shouldClear) return;

  const currentId = state.currentExercise?.id;
  state.progress = { answers: {}, reports: {}, history: {}, lastExerciseId: currentId ?? null };
  saveProgress();

  renderCatalog();
  renderDashboard();
  renderHome();

  if (currentId && !elements.exerciseView.hidden) {
    await selectExercise(currentId);
  }
});

function openSolutionModal() {
  const { currentExercise } = state;
  if (!currentExercise?.guide) return;

  elements.solutionTitle.textContent = currentExercise.title;
  elements.solutionContent.replaceChildren(renderGuide(currentExercise.guide));
  elements.solutionModal.hidden = false;
  elements.closeSolutionButton.focus();
}

function closeSolutionModal() {
  elements.solutionModal.hidden = true;
  elements.answerInput.focus();
}

function renderGuide(guide) {
  const wrapper = document.createElement("div");
  wrapper.className = "guide-grid";

  const intro = document.createElement("section");
  intro.className = "guide-section guide-intro";
  const summary = document.createElement("p");
  summary.textContent = guide.summary;
  intro.append(summary);

  const solution = document.createElement("section");
  solution.className = "guide-section";
  const solutionTitle = document.createElement("h4");
  solutionTitle.textContent = "Solucao sugerida";
  const warning = document.createElement("div");
  warning.className = "copy-warning";
  warning.setAttribute("role", "status");
  warning.hidden = true;

  const warningTitle = document.createElement("strong");
  warningTitle.textContent = COPY_WARNING.split("! ")[0] + "!";
  const warningText = document.createElement("span");
  warningText.textContent = COPY_WARNING.split("! ")[1];
  warning.append(warningTitle, warningText);

  const code = document.createElement("pre");
  code.className = "solution-code";
  code.innerHTML = highlightRust(guide.solution);
  code.addEventListener("copy", blockSolutionCopy);
  code.addEventListener("cut", blockSolutionCopy);
  solution.append(solutionTitle, warning, code);

  const concepts = renderGuideList("O que esta sendo praticado", guide.concepts);
  const pitfalls = renderGuideList("Cuidados comuns", guide.pitfalls);

  const docs = document.createElement("section");
  docs.className = "guide-section";
  const docsTitle = document.createElement("h4");
  docsTitle.textContent = "Docs oficiais";
  const docsList = document.createElement("div");
  docsList.className = "doc-links";
  for (const doc of guide.docs ?? []) {
    const link = document.createElement("a");
    link.href = doc.url;
    link.target = "_blank";
    link.rel = "noreferrer";
    link.textContent = doc.title;
    docsList.append(link);
  }
  docs.append(docsTitle, docsList);

  wrapper.append(intro, solution, concepts, pitfalls, docs);
  return wrapper;
}

function renderRubric(exercise) {
  elements.rubricPanel.replaceChildren();
  elements.rubricPanel.hidden = !state.settings.showRubric;
  if (!state.settings.showRubric) return;

  const title = document.createElement("strong");
  title.textContent = "Checklist antes de verificar";
  const list = document.createElement("ul");

  for (const item of exercise.rubric ?? []) {
    const li = document.createElement("li");
    li.textContent = checklistText(item);
    list.append(li);
  }

  elements.rubricPanel.append(title, list);
}

function checklistText(item) {
  const id = item.id.toLowerCase();

  if (id.includes("hashmap")) return "Use HashMap para representar pares chave-valor.";
  if (id.includes("hashset")) return "Use HashSet para manter valores unicos.";
  if (id.includes("vecdeque")) return "Use VecDeque quando precisar de comportamento de fila.";
  if (id.includes("btreemap")) return "Use BTreeMap quando a ordem das chaves importar.";
  if (id.includes("stack")) return "Use uma colecao como pilha, adicionando e removendo pelo fim.";
  if (id.includes("insert")) return "Use o metodo de insercao da colecao.";
  if (id.includes("get")) return "Use o metodo de consulta da colecao.";
  if (id.includes("push")) return "Adicione elementos usando o metodo apropriado.";
  if (id.includes("pop")) return "Remova o ultimo item usando o metodo apropriado.";
  if (id.includes("for")) return "Use iteracao explicita para percorrer os dados.";
  if (id.includes("while")) return "Use repeticao controlada por condicao.";
  if (id.includes("loop")) return "Use repeticao aberta quando precisar controlar a saida manualmente.";
  if (id.includes("if")) return "Use uma decisao condicional.";
  if (id.includes("match")) return "Trate as alternativas de forma explicita.";
  if (id.includes("fn") || id.includes("func")) return "Separe a logica em uma funcao.";
  if (id.includes("param")) return "Defina os parametros esperados.";
  if (id.includes("return") || id.includes("retorna")) return "Declare ou produza o valor de retorno esperado.";
  if (id.includes("read")) return "Use uma operacao de leitura de arquivo.";
  if (id.includes("write")) return "Use uma operacao de escrita de arquivo.";
  if (id.includes("append")) return "Configure a escrita para adicionar ao final.";
  if (id.includes("path")) return "Use a API de caminhos da biblioteca padrao.";
  if (id.includes("question")) return "Propague erros em vez de ignorar falhas.";
  if (id.includes("unwrap")) return "Defina um caminho seguro para ausencia ou erro.";
  if (id.includes("tipo") || id.includes("type")) return "Deixe claro o tipo que o exercicio pede.";

  return item.text
    .replace(/^Esperava\s+/i, "")
    .replace(/^Este exercício pede\s+/i, "")
    .replace(/^Este exercicio pede\s+/i, "")
    .replace(/\b[A-Za-z_][A-Za-z0-9_:]*\s*(?:\(|::|<|\.)[^ ]*/g, "a API apropriada")
    .replace(/\.$/, "");
}

function renderGuideList(titleText, items = []) {
  const section = document.createElement("section");
  section.className = "guide-section";

  const title = document.createElement("h4");
  title.textContent = titleText;

  const list = document.createElement("ul");
  for (const item of items) {
    const li = document.createElement("li");
    li.textContent = item;
    list.append(li);
  }

  section.append(title, list);
  return section;
}

function blockSolutionCopy(event) {
  event.preventDefault();
  window.getSelection()?.removeAllRanges();
  showCopyWarning();
}

function showCopyWarning() {
  const warning = elements.solutionContent.querySelector(".copy-warning");
  if (!warning) return;

  warning.hidden = false;
  warning.classList.remove("show");
  requestAnimationFrame(() => warning.classList.add("show"));

  window.clearTimeout(state.copyWarningTimer);
  state.copyWarningTimer = window.setTimeout(() => {
    warning.classList.remove("show");
    warning.hidden = true;
  }, 2600);
}

function selectionTouchesSolutionCode() {
  const selection = window.getSelection();
  if (!selection || selection.rangeCount === 0) return false;

  const codeBlock = elements.solutionContent.querySelector(".solution-code");
  if (!codeBlock) return false;

  for (let index = 0; index < selection.rangeCount; index += 1) {
    const range = selection.getRangeAt(index);
    if (rangeIntersectsNode(range, codeBlock)) {
      return true;
    }
  }

  return false;
}

function solutionCodeSelectionLength() {
  const selection = window.getSelection();
  if (!selection || selection.rangeCount === 0) return 0;

  const codeBlock = elements.solutionContent.querySelector(".solution-code");
  if (!codeBlock) return 0;

  let selectedText = "";
  for (let index = 0; index < selection.rangeCount; index += 1) {
    const range = selection.getRangeAt(index);
    if (rangeIntersectsNode(range, codeBlock)) {
      selectedText += range.toString();
    }
  }

  return selectedText.length;
}

function rangeIntersectsNode(range, node) {
  if (typeof range.intersectsNode === "function") {
    return range.intersectsNode(node);
  }

  return node.contains(range.commonAncestorContainer);
}

elements.hintButton.addEventListener("click", async () => {
  if (!state.currentExercise) return;

  try {
    const hint = await invoke("get_hint", {
      id: state.currentExercise.id,
      index: state.nextHint,
    });

    renderHint(hint);
    state.nextHint = Math.min(hint.total, state.nextHint + 1);
  } catch (error) {
    renderHint({
      index: state.currentExercise.hint_count,
      total: state.currentExercise.hint_count,
      text: String(error),
    });
  }
});

elements.checkButton.addEventListener("click", async () => {
  if (!state.currentExercise) return;

  setStatus("Verificando", "busy");
  elements.checkButton.disabled = true;

  try {
    const report = await invoke("check_answer", {
      id: state.currentExercise.id,
      answer: elements.answerInput.value,
    });

    renderReport(report);
  } catch (error) {
    renderFatal(String(error));
  } finally {
    elements.checkButton.disabled = false;
  }
});

elements.answerInput.addEventListener("input", () => {
  persistAnswer();
  syncHighlight();
});

elements.answerInput.addEventListener("scroll", syncEditorScroll);

elements.answerInput.addEventListener("keydown", (event) => {
  if (event.key !== "Tab") return;
  event.preventDefault();

  const start = elements.answerInput.selectionStart;
  const end = elements.answerInput.selectionEnd;
  const value = elements.answerInput.value;
  elements.answerInput.value = `${value.slice(0, start)}    ${value.slice(end)}`;
  elements.answerInput.selectionStart = elements.answerInput.selectionEnd = start + 4;
  persistAnswer();
  syncHighlight();
});

function persistAnswer() {
  if (!state.currentExercise) return;
  state.progress.answers[state.currentExercise.id] = elements.answerInput.value;
  state.progress.lastExerciseId = state.currentExercise.id;
  saveProgress();
  elements.saveStateLabel.textContent = "Salvo localmente";
}

function renderHint(hint) {
  elements.hintPanel.hidden = false;

  const item = document.createElement("article");
  item.className = "hint-item";

  const label = document.createElement("span");
  label.className = "hint-label";
  label.textContent = `Dica ${hint.index}/${hint.total}`;

  const text = document.createElement("p");
  text.textContent = hint.text;

  item.append(label, text);
  elements.hintPanel.append(item);
}

function renderReport(report, options = { persist: true }) {
  elements.resultOutput.replaceChildren();
  setStatus(report.passed ? "Aprovado" : "Ajustar", report.passed ? "passed" : "failed");

  if (options.persist) {
    state.progress.reports[report.exercise_id] = {
      ...report,
      checked_at: new Date().toISOString(),
    };
    appendHistory(report);
    saveProgress();
    renderCatalog();
    renderDashboard();
    renderHome();
    renderActiveExercise();
  }

  const saved = state.progress.reports[report.exercise_id] ?? report;
  elements.lastRunLabel.textContent = saved.checked_at
    ? `Ultima verificacao: ${formatTime(saved.checked_at)}`
    : "Resultado salvo";

  if (report.compile) {
    elements.resultOutput.append(
      renderCheckRow({
        ok: report.compile.ok,
        message: report.compile.ok
          ? "Codigo compila."
          : report.compile.timed_out
            ? "Compilacao excedeu o tempo limite."
            : "Codigo nao compila.",
        detail: report.compile.stderr,
      }),
    );
  }

  for (const check of report.checks) {
    elements.resultOutput.append(renderCheckRow(check));
  }
  renderHistory(report.exercise_id);
}

function appendHistory(report) {
  const id = report.exercise_id;
  const entry = {
    passed: report.passed,
    checked_at: new Date().toISOString(),
    compile_ok: report.compile?.ok ?? null,
    failed_checks: report.checks.filter((check) => !check.ok).length,
  };

  state.progress.history[id] = [entry, ...(state.progress.history[id] ?? [])].slice(0, 8);
}

function renderHistory(id) {
  elements.historyPanel.replaceChildren();
  elements.historyPanel.hidden = !state.settings.showHistory;
  if (!state.settings.showHistory) return;

  const entries = state.progress.history[id] ?? [];
  const title = document.createElement("strong");
  title.textContent = "Historico";
  elements.historyPanel.append(title);

  if (entries.length === 0) {
    const empty = document.createElement("p");
    empty.textContent = "Nenhuma tentativa registrada ainda.";
    elements.historyPanel.append(empty);
    return;
  }

  for (const entry of entries) {
    const item = document.createElement("article");
    item.className = `history-item ${entry.passed ? "ok" : "fail"}`;
    const status = entry.passed ? "Aprovado" : `${entry.failed_checks} ajuste(s)`;
    item.textContent = `${formatTime(entry.checked_at)} • ${status}`;
    elements.historyPanel.append(item);
  }
}

function renderCheckRow(check) {
  const item = document.createElement("article");
  item.className = `check-row ${check.ok ? "ok" : "fail"}`;

  const marker = document.createElement("span");
  marker.className = "check-marker";
  marker.textContent = check.ok ? "OK" : "!";

  const content = document.createElement("div");
  const message = document.createElement("p");
  message.textContent = check.message;
  content.append(message);

  if (check.detail) {
    const detail = check.detail.includes("\n")
      ? document.createElement("pre")
      : document.createElement("small");
    detail.textContent = check.detail;
    content.append(detail);
  }

  item.append(marker, content);
  return item;
}

function renderEmptyResult(summary) {
  const empty = document.createElement("div");
  empty.className = "empty-result";

  const title = document.createElement("strong");
  title.textContent = summary
    ? `Exercicio: ${summary.title}`
    : "Selecione um exercicio";

  const text = document.createElement("p");
  text.textContent = "Escreva a resposta e verifique para registrar o progresso.";

  empty.append(title, text);
  return empty;
}

function renderDashboard() {
  const visible = visibleExercises();
  const total = visible.length;
  const completed = visible.filter((exercise) => isCompleted(exercise.id)).length;
  const percent = total === 0 ? 0 : Math.round((completed / total) * 100);

  elements.progressPercent.textContent = `${percent}%`;
  elements.progressCount.textContent = `${completed} de ${total} concluidos`;
  elements.progressDetail.textContent =
    completed === total && total > 0
      ? "Todos os exercicios aprovados."
      : `${Math.max(total - completed, 0)} restantes`;

  document.documentElement.style.setProperty("--progress", `${percent * 3.6}deg`);
  elements.homeCompleted.textContent = completed.toString();
}

function visibleExercises() {
  return state.flatExercises.filter((exercise) => {
    if (state.settings.reviewMode && isCompleted(exercise.id)) return false;
    if (!state.searchTerm) return true;

    const haystack = [
      exercise.title,
      exercise.module_title,
      exercise.difficulty,
      exercise.id,
    ].join(" ").toLowerCase();
    return haystack.includes(state.searchTerm);
  });
}

function syncSettingsControls() {
  elements.flagRubric.checked = state.settings.showRubric;
  elements.flagHistory.checked = state.settings.showHistory;
  elements.flagReview.checked = state.settings.reviewMode;
  elements.flagTooltips.checked = state.settings.tooltips;
}

function applySettings() {
  syncSettingsControls();
  document.body.classList.toggle("tooltips-disabled", !state.settings.tooltips);
  renderCatalog();
  renderDashboard();
  renderHome();
  if (state.currentExercise) {
    renderRubric(state.currentExercise);
    renderHistory(state.currentExercise.id);
    refreshExercisePosition();
  }
}

function isCompleted(id) {
  return state.progress.reports[id]?.passed === true;
}

function exerciseState(id) {
  if (isCompleted(id)) return "done";
  if (state.progress.reports[id]) return "attempted";
  if ((state.progress.answers[id] ?? "").trim()) return "draft";
  return "empty";
}

function findSummary(id) {
  return state.flatExercises.find((exercise) => exercise.id === id);
}

function difficultyRank(difficulty) {
  if (difficulty === "Iniciante") return 0;
  if (difficulty === "Intermediario") return 1;
  return 2;
}

function setStatus(text, stateName) {
  elements.statusBadge.textContent = text;
  elements.statusBadge.dataset.state = stateName;
}

function showCatalogStatus(text, stateName) {
  elements.catalogStatus.textContent = text;
  elements.catalogStatus.dataset.state = stateName;
  elements.catalogStatus.hidden = false;

  window.clearTimeout(state.catalogStatusTimer);
  if (stateName !== "busy") {
    state.catalogStatusTimer = window.setTimeout(() => {
      elements.catalogStatus.hidden = true;
    }, 3200);
  }
}

function syncHighlight() {
  const value = elements.answerInput.value || " ";
  elements.highlightLayer.innerHTML = highlightRust(value);
  syncEditorScroll();
}

function syncEditorScroll() {
  elements.highlightLayer.scrollTop = elements.answerInput.scrollTop;
  elements.highlightLayer.scrollLeft = elements.answerInput.scrollLeft;
}

function highlightRust(source) {
  const escaped = escapeHtml(source);
  const tokenPattern =
    /(\/\/.*|"(?:\\.|[^"\\])*"|\b(?:fn|let|mut|for|in|if|else|match|while|loop|return|pub|struct|enum|impl|use|mod|crate|self|Self|as|const|static|move|ref|true|false)\b|\b(?:Vec|String|Option|Result|Some|None|Ok|Err|HashMap|HashSet|VecDeque|BTreeMap|Path|OpenOptions|i32|u32|usize|u128|str|bool)\b|\b\d+(?:_\d+)*\b|\b[a-zA-Z_][a-zA-Z0-9_]*!)/g;

  return escaped.replace(tokenPattern, (token) => {
    if (token.startsWith("//")) return `<span class="tok-comment">${token}</span>`;
    if (token.startsWith('"')) return `<span class="tok-string">${token}</span>`;
    if (/^\d/.test(token)) return `<span class="tok-number">${token}</span>`;
    if (token.endsWith("!")) return `<span class="tok-macro">${token}</span>`;
    if (/^(Vec|String|Option|Result|Some|None|Ok|Err|HashMap|HashSet|VecDeque|BTreeMap|Path|OpenOptions|i32|u32|usize|u128|str|bool)$/.test(token)) {
      return `<span class="tok-type">${token}</span>`;
    }
    return `<span class="tok-keyword">${token}</span>`;
  });
}

function escapeHtml(value) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

function formatTime(iso) {
  return new Intl.DateTimeFormat("pt-BR", {
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(iso));
}

function renderFatal(message) {
  showExerciseView();
  elements.exerciseTitle.textContent = "Erro";
  elements.exercisePrompt.textContent = message;
  setStatus("Erro", "failed");
}
